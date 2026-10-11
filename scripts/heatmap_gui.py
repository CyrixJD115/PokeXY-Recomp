#!/usr/bin/env python3
"""RDComp live heatmap GUI — single-page dashboard for the Pokémon X/Y
recomp/decomp progress, with PNG/SVG export.

Data model (all machine-local, produced from your own dump):

  stage 0  Raw Binary        function exists in the 3dsrecomp port, nothing
                             else known about it
  stage 1  Identified/Named  carries a real symbol — CRO export table
                             (cro_extracted/<Mod>/symbols.tsv), static.crs
                             exports (cro_extracted/static/symbols.tsv), or a
                             non-synthetic Ghidra name (pipeline/symbols/*.csv)
  stage 2  Decompiled to C   a Ghidra decompiled .c exists in the decomp
                             corpus (decomp/code_c + decomp/cro_c/<Mod>)
  stage 3  Natively Verified recompiled code byte/exact-verified against the
                             original (per-function: --verified list; module
                             level: byte-identical relink in pipeline/out)

  overlay  Interpreted      address appears in a Zakuro interpreter hints
           (hints)          file (--hints) — the AOT recomp didn't cover it

  stat     Recomp-covered   the 3dsrecomp port has a function start here
           (not coloured)   (bit 16; shown in tooltips/module/global stats)

The cell grid is the UNION of every tool's function starts (3dsrecomp port
headers, Ghidra symbol CSVs, the decompiled-C corpus, export tables) — the
port merges some functions Ghidra splits and vice versa, so no single
source is authoritative. Cell colour encodes the highest achieved stage;
the unnamed-decompiled mass is teal, named green, verified gold, raw dark
grey, hints red.

Coordinates: Main (functions.h) uses absolute 0x100000-based addresses,
matching code.bin.csv and static.crs load_addr. CRO modules
(m<NNN>_functions.h) use module-relative text offsets (0x180-based),
matching the shifted <Dll>.cro.csv rows AND the `offset` column of the
CRO symbols.tsv; the cro_c decomp corpus filenames are load-address based
(0x100000 + offset), so they are rebased by -0x100000 before matching.

Usage:
  python3 scripts/heatmap_gui.py                      # one-shot build
  python3 scripts/heatmap_gui.py --watch              # rebuild on change
  python3 scripts/heatmap_gui.py --serve 8765         # serve + live page
  python3 scripts/heatmap_gui.py --hints hints.txt    # overlay interp hits

Outputs (default .Data/Source_Extracted/heatmap_gui/):
  heatmap.html        self-contained page (data embedded; works from file://)
  heatmap_data.json   same payload; the page polls it when served over http
  summary.md          plain-text per-module table (old heatmap.py parity)

The page polls heatmap_data.json every 3 s when loaded over http(s) and
re-renders — pair --watch with --serve for a fully live dashboard.
"""
import argparse
import csv
import glob
import hashlib
import html
import json
import os
import re
import sys
import time

MOD_RE = re.compile(r'\{"([^"]+)", &m(\d+)_base, (0x[0-9A-Fa-f]+)u, (\d+),')
FN_RE = re.compile(r'void (?:m\d+_)?f_([0-9A-Fa-f]+)\(Context')

# Ghidra synthetic name families — these are NOT "identified" functions.
SYNTHETIC = ("FUN_", "thunk_FUN_", "DAT_", "UNK_", "LAB_", "SUB_",
             "switchD_", "switchdataD_", "module_entry", "module_")

MANGLED_NS_RE = re.compile(r'^_ZN([0-9]+)')


def parse_module_map(entries_c):
    mods = {}
    with open(entries_c) as f:
        for line in f:
            m = MOD_RE.search(line)
            if m:
                mods[int(m.group(2))] = {
                    "name": m.group(1),
                    "size": int(m.group(3), 16),
                    "declared": int(m.group(4)),
                }
    return mods


def parse_functions(path):
    addrs = set()
    with open(path) as f:
        for line in f:
            m = FN_RE.search(line)
            if m:
                addrs.add(int(m.group(1), 16) & ~1)
    return sorted(addrs)


def resolve_ci(directory, filename):
    """Case-insensitive path resolution (entries.c says DllEShop, the
    pipeline files say DllEshop — both spellings occur)."""
    target = os.path.join(directory, filename)
    if os.path.exists(target):
        return target
    try:
        low = filename.lower()
        for entry in os.listdir(directory):
            if entry.lower() == low:
                return os.path.join(directory, entry)
    except OSError:
        pass
    return target


def parse_symbols_csv(csv_path):
    """Ghidra symbol CSV -> (function starts, {addr&~1: real name})."""
    starts, named = set(), {}
    try:
        with open(csv_path) as f:
            for row in csv.DictReader(f):
                name = row["Name"].strip('"')
                try:
                    addr = int(row["Location"].strip('"'), 16) & ~1
                except ValueError:
                    continue
                if name.startswith(("FUN_", "thunk_FUN_")):
                    starts.add(addr)
                elif not name.startswith(SYNTHETIC):
                    named[addr] = name
                    starts.add(addr)
    except FileNotFoundError:
        pass
    return starts, named


def parse_symbols_tsv(tsv_path):
    """CRO/static export table -> {offset&~1: name} (real game symbols)."""
    out = {}
    try:
        with open(tsv_path) as f:
            next(f, None)
            for line in f:
                parts = line.rstrip("\n").split("\t")
                if len(parts) >= 2 and parts[0]:
                    try:
                        out[int(parts[1], 16) & ~1] = parts[0]
                    except ValueError:
                        pass
    except FileNotFoundError:
        pass
    return out


def parse_decomp_dir(path, rebase=0):
    """Set of addresses covered by the Ghidra decompiled C corpus."""
    out = set()
    for fn in glob.glob(os.path.join(path, "*.c")):
        base = os.path.basename(fn).split("_", 1)[0]
        try:
            out.add((int(base, 16) - rebase) & ~1)
        except ValueError:
            pass
    return out


def parse_hints(path):
    addrs = set()
    if not path:
        return addrs
    started = False
    for line in open(path):
        line = line.strip()
        if line.startswith("="):
            started = True
            continue
        if started and line and not line.startswith(("#", "/")):
            try:
                addrs.add(int(line, 16) & ~1)
            except ValueError:
                pass
    return addrs


def parse_verified(path):
    """Optional JSON list of {module, addr} entries verified natively."""
    out = {}
    if not path:
        return out
    for rec in json.load(open(path)):
        out.setdefault(rec.get("module", ""), set()).add(
            int(rec["addr"], 16) & ~1)
    return out


def demangle_namespace(name):
    """Extract a '::' namespace from an Itanium-mangled name (light parse).

    Components are length-prefixed: _ZN9xy_system2bg6System13SetIsCallDrawEb
    -> xy_system::bg::System (last component = the function name).
    """
    if not name.startswith("_ZN"):
        return ""
    rest = name[3:]
    parts = []
    while rest:
        m = re.match(r'([0-9]+)', rest)
        if not m:
            return ""
        n = int(m.group(1))
        start = len(m.group(1))
        if n == 0 or len(rest) < start + n:
            return ""
        parts.append(rest[start:start + n])
        rest = rest[start + n:]
        if not rest or not rest[0].isdigit():
            break
    return "::".join(parts[:-1]) if len(parts) > 1 else ""


def parse_imports(tsv_path):
    """imports.tsv -> list of imported module names."""
    mods = []
    try:
        with open(tsv_path) as f:
            next(f, None)
            for line in f:
                parts = line.rstrip("\n").split("\t")
                if parts and parts[0]:
                    mods.append(parts[0].strip("|"))
    except FileNotFoundError:
        pass
    return mods


def sha256_of(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def module_link_verified(pipeline_dir, module_file):
    """Byte-identical relink check: pipeline/orig/<f> vs pipeline/out/<f>."""
    orig = os.path.join(pipeline_dir, "orig", module_file)
    out = os.path.join(pipeline_dir, "out", module_file)
    try:
        if os.path.getsize(orig) != os.path.getsize(out):
            return False
        return sha256_of(orig) == sha256_of(out)
    except (FileNotFoundError, OSError):
        return False


def collect(args):
    src = args.source_root
    recomp = args.recomp or os.path.join(src, "recomp")
    symbols_dir = args.symbols or os.path.join(src, "pipeline", "symbols")
    cro_dir = args.cro or os.path.join(src, "cro_extracted")
    decomp = args.decomp or os.path.join(src, "decomp")
    pipeline = args.pipeline or os.path.join(src, "pipeline")

    mods = parse_module_map(os.path.join(recomp, "entries.c"))
    hints = parse_hints(args.hints)
    verified = parse_verified(args.verified)

    # Main executable: absolute addresses; static.crs = its export table.
    plan = [("Main (code.bin)", "code.bin", 0,
             os.path.join(recomp, "functions.h"),
             os.path.join(symbols_dir, "code.bin.csv"),
             os.path.join(cro_dir, "static", "symbols.tsv"),
             os.path.join(decomp, "code_c"), 0)]
    # CROs: m<NNN>_functions.h + <Dll>.cro.csv + exports, module-relative.
    for mid, info in sorted(mods.items()):
        h = os.path.join(recomp, f"m{mid:03d}_functions.h")
        if os.path.exists(h):
            plan.append((info["name"], info["name"], mid, h,
                         resolve_ci(symbols_dir, f"{info['name']}.cro.csv"),
                         os.path.join(cro_dir, info["name"], "symbols.tsv"),
                         resolve_ci(os.path.join(decomp, "cro_c"),
                                    info["name"]),
                         0x100000))

    ns_index = {}          # ns string -> id
    ns_counts = {}
    modules = []
    totals = {"functions": 0, "named": 0, "decompiled": 0, "verified": 0,
              "hints": 0, "recomp": 0}
    importer_index = {}     # module -> [importers]

    for name, modfile, mid, hpath, csv_path, tsv_path, cdir, rebase in plan:
        port = set(parse_functions(hpath))          # 3dsrecomp's view
        gh_starts, names = parse_symbols_csv(csv_path)
        names.update(parse_symbols_tsv(tsv_path))   # real exports win
        decomp_set = parse_decomp_dir(cdir, rebase)
        ver = verified.get(name, verified.get(modfile, set()))

        # Function inventory = union of every tool's function starts. The
        # port merges some functions Ghidra splits (and splits some Ghidra
        # merges), so no single source is authoritative.
        funcs = sorted(port | gh_starts | decomp_set | set(names) | ver)

        imports = parse_imports(resolve_ci(
            os.path.join(cro_dir, "static" if mid == 0 else name),
            "imports.tsv"))
        for imp in imports:
            importer_index.setdefault(imp, []).append(name)

        idx_of = {a: i for i, a in enumerate(funcs)}
        st = []
        names_j = {}
        ns_j = {}
        for a in funcs:
            s = 0
            if a in names:
                s |= 1
            if a in decomp_set:
                s |= 2
            if a in ver:
                s |= 4
            if a in hints:
                s |= 8
            if a in port:
                s |= 16                    # recomp-covered (stats only)
            st.append(s)
            if a in names:
                i = idx_of[a]
                nm = names[a]
                names_j[str(i)] = nm
                ns = demangle_namespace(nm)
                if ns:
                    ns_j[str(i)] = ns_index.setdefault(ns, len(ns_index))
                    ns_counts[ns] = ns_counts.get(ns, 0) + 1

        totals["functions"] += len(funcs)
        totals["named"] += sum(1 for s in st if s & 1)
        totals["decompiled"] += sum(1 for s in st if s & 2)
        totals["verified"] += sum(1 for s in st if s & 4)
        totals["hints"] += sum(1 for s in st if s & 8)
        totals["recomp"] += sum(1 for s in st if s & 16)

        link_ok = module_link_verified(
            pipeline, modfile if mid == 0 else
            os.path.basename(resolve_ci(os.path.join(pipeline, "orig"),
                                        f"{modfile}.cro")))

        modules.append({
            "name": name, "mid": mid, "n": len(funcs),
            "addrs": " ".join(f"{a:08x}" for a in funcs),
            "st": st, "names": names_j, "ns": ns_j,
            "imports": imports, "linkOk": link_ok,
        })

    for m in modules:
        m["importers"] = sorted(importer_index.get(m["name"], []))

    namespaces = [{"ns": ns, "count": ns_counts[ns]}
                  for ns in sorted(ns_counts, key=lambda k: -ns_counts[k])]
    link_total = sum(1 for m in modules if m["linkOk"])
    return {
        "generated": time.strftime("%Y-%m-%d %H:%M:%S"),
        "game": "Pokémon X (recomp basis: title 0004000000055D00)",
        "totals": {**totals, "modules": len(modules),
                   "linkVerified": link_total},
        "namespaces": namespaces,
        "nsIndex": [None] * len(ns_index) if not ns_index else
                   sorted(ns_index, key=ns_index.get),
        "modules": modules,
    }


def render_summary(data, out_dir):
    rows = ["| module | functions | named | named % | decompiled | recomp | "
            "verified | hints | link-ok |",
            "|---|---:|---:|---:|---:|---:|---:|---:|---|"]
    for m in data["modules"]:
        named = sum(1 for s in m["st"] if s & 1)
        dec = sum(1 for s in m["st"] if s & 2)
        rec = sum(1 for s in m["st"] if s & 16)
        ver = sum(1 for s in m["st"] if s & 4)
        hint = sum(1 for s in m["st"] if s & 8)
        pct = 100.0 * named / max(m["n"], 1)
        rows.append(f"| {m['name']} | {m['n']} | {named} | {pct:.1f}% | "
                    f"{dec} | {rec} | {ver} | {hint} | "
                    f"{'yes' if m['linkOk'] else '—'} |")
    t = data["totals"]
    with open(os.path.join(out_dir, "summary.md"), "w") as f:
        f.write("# RDComp heatmap\n\n" + "\n".join(rows))
        f.write(f"\n\n**Total: {t['functions']} functions, {t['named']} named "
                f"({100.0 * t['named'] / max(t['functions'], 1):.1f}%), "
                f"{t['decompiled']} decompiled, {t['recomp']} recomp-covered, "
                f"{t['verified']} verified, {t['hints']} interpreter hits, "
                f"{t['linkVerified']}/{t['modules']} modules link-verified.**\n")


HTML_TEMPLATE = r"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>PokeXYZ-RDComp · decomp/recomp heatmap</title>
<style>
:root{
  --bg:#0f1115;--panel:#16181d;--panel2:#1b1e24;--border:#23262d;
  --text:#e8e8e8;--dim:#9aa0aa;--accent:#4fc3f7;
  --c-raw:#414650;--c-decomp:#2b6f8a;--c-named:#2ecc71;
  --c-verified:#f0b429;--c-hint:#e74c3c;
}
*{box-sizing:border-box}
body{margin:0;background:var(--bg);color:var(--text);
  font:14px/1.45 "Segoe UI",system-ui,sans-serif}
.mono{font-family:ui-monospace,"Cascadia Mono",Consolas,monospace}
header{position:sticky;top:0;z-index:50;background:linear-gradient(
  180deg,#14171c 0%,#14171cf2 100%);border-bottom:1px solid var(--border);
  padding:10px 18px;display:flex;flex-wrap:wrap;gap:10px 22px;align-items:center}
header h1{font-size:16px;margin:0;font-weight:600;letter-spacing:.3px}
header h1 small{color:var(--dim);font-weight:400;margin-left:8px}
.chips{display:flex;gap:8px;flex-wrap:wrap}
.chip{background:var(--panel2);border:1px solid var(--border);border-radius:6px;
  padding:3px 9px;font-size:12px;color:var(--dim)}
.chip b{color:var(--text);font-weight:600}
.spacer{flex:1}
.btn{background:#20242c;border:1px solid #2f343e;border-radius:6px;color:var(--text);
  padding:5px 11px;font-size:12px;cursor:pointer}
.btn:hover{background:#282d37;border-color:var(--accent)}
.live{display:inline-flex;align-items:center;gap:6px;font-size:12px;color:var(--dim)}
.live .dot{width:8px;height:8px;border-radius:50%;background:#555;transition:.3s}
.live.on .dot{background:#2ecc71;box-shadow:0 0 6px #2ecc71;animation:pulse 2s infinite}
@keyframes pulse{50%{opacity:.5}}
#layout{display:flex;min-height:calc(100vh - 58px)}
#side{width:270px;min-width:270px;border-right:1px solid var(--border);
  padding:12px;position:sticky;top:58px;height:calc(100vh - 58px);overflow-y:auto}
#side input,#side select{width:100%;background:var(--panel2);color:var(--text);
  border:1px solid var(--border);border-radius:6px;padding:6px 8px;font-size:12px;
  margin-bottom:8px}
.modrow{padding:6px 8px;border-radius:6px;cursor:pointer;display:flex;
  flex-wrap:wrap;justify-content:space-between;gap:2px 6px;align-items:baseline}
.modrow:hover{background:var(--panel2)}
.modrow .nm{font-size:12px;overflow:hidden;text-overflow:ellipsis;
  white-space:nowrap;flex:1;min-width:0}
.modrow .pct{font-size:11px;color:var(--dim);white-space:nowrap}
.bar{height:4px;border-radius:2px;background:var(--c-raw);overflow:hidden;
  display:flex;margin-top:3px;flex-basis:100%}
.bar i{display:block;height:100%}
main{flex:1;padding:16px 20px 60px;max-width:1200px}
h2{font-size:13px;color:var(--dim);text-transform:uppercase;letter-spacing:1px;
  margin:22px 0 8px}
#overview .stackbar{display:flex;height:22px;border-radius:5px;overflow:hidden;
  border:1px solid var(--border)}
#overview .stackbar i{display:block;height:100%}
#overview .legend{display:flex;gap:14px;flex-wrap:wrap;margin-top:8px;font-size:12px;
  color:var(--dim)}
#overview .legend .sw{display:inline-block;width:10px;height:10px;
  border-radius:2px;margin-right:5px;vertical-align:-1px}
#nslist{display:flex;flex-wrap:wrap;gap:6px}
.nschip{background:var(--panel2);border:1px solid var(--border);border-radius:14px;
  padding:3px 11px;font-size:12px;cursor:pointer;color:var(--dim)}
.nschip:hover{border-color:var(--accent);color:var(--text)}
.nschip.sel{border-color:#2ecc71;color:#2ecc71}
#nsfilter{display:none;margin:8px 0;padding:7px 12px;background:#182417;
  border:1px solid #2e5b2e;border-radius:6px;font-size:12px}
section.mod{margin-bottom:26px;border:1px solid var(--border);border-radius:10px;
  background:var(--panel);overflow:hidden}
section.mod>header{position:static;background:var(--panel2);border-bottom:
  1px solid var(--border);padding:8px 14px;display:flex;flex-wrap:wrap;
  gap:6px 14px;align-items:center}
section.mod h3{margin:0;font-size:14px;font-weight:600}
section.mod .meta{font-size:11px;color:var(--dim)}
section.mod .meta b{color:var(--text)}
section.mod .mini{display:flex;gap:2px;align-items:center}
section.mod .mini i{width:9px;height:9px;border-radius:2px}
section.mod canvas{display:block;margin:10px 14px 14px;cursor:crosshair;
  border-radius:4px}
section.mod .imports{padding:0 14px 12px;display:flex;flex-wrap:wrap;gap:5px}
.imp{font-size:11px;background:var(--panel2);border:1px solid var(--border);
  border-radius:4px;padding:2px 7px;cursor:pointer;color:var(--dim)}
.imp:hover{color:var(--accent);border-color:var(--accent)}
#tooltip{position:fixed;z-index:99;background:#0b0d10f2;border:1px solid #2f343e;
  border-radius:6px;padding:7px 10px;font-size:12px;pointer-events:none;
  display:none;max-width:480px}
#tooltip .t1{color:var(--accent)}#tooltip .t2{color:var(--dim)}
footer{color:var(--dim);font-size:11px;margin-top:30px;border-top:
  1px solid var(--border);padding-top:10px}
@media(max-width:860px){#side{display:none}}
</style>
</head>
<body>
<header>
  <h1>PokeXYZ-RDComp heatmap<small class="mono" id="game"></small></h1>
  <div class="chips" id="stats"></div>
  <div class="spacer"></div>
  <span class="live" id="live"><span class="dot"></span><span id="livetxt">
    static</span></span>
  <span class="chip mono" id="stamp"></span>
  <button class="btn" id="expSvgAll">Export SVG (all)</button>
  <button class="btn" id="expPngAll">Export PNG (all)</button>
</header>
<div id="layout">
  <aside id="side">
    <input id="q" placeholder="filter modules…" spellcheck="false">
    <select id="sort">
      <option value="name">sort: name</option>
      <option value="funcs">sort: functions ↓</option>
      <option value="named">sort: named % ↓</option>
      <option value="unnamed">sort: unnamed ↓</option>
    </select>
    <div id="modlist"></div>
  </aside>
  <main>
    <div id="overview">
      <div class="stackbar" id="stackbar"></div>
      <div class="legend" id="legend"></div>
    </div>
    <h2>Namespaces (identified symbols)</h2>
    <div id="nsfilter"></div>
    <div id="nslist"></div>
    <h2>Modules — address-ordered function grid</h2>
    <div id="mods"></div>
    <footer>Cell = one function, address order left→right, top→bottom.
      Colour = highest achieved stage. Hover for details; click a module
      name in the sidebar to jump. Sources: 3dsrecomp port headers ·
      pipeline symbol CSVs · CRO/static export tables · Ghidra decomp
      corpus · pipeline byte-identical relinks.</footer>
  </main>
</div>
<div id="tooltip"></div>
<script>
"use strict";
const DATA = __DATA_JSON__;

/* ---- stage model ---- */
const STAGES = [
  {bit:8, key:"hint",     label:"Interpreted (hints)",  color:"#e74c3c"},
  {bit:4, key:"verified", label:"Natively Verified",    color:"#f0b429"},
  {bit:1, key:"named",    label:"Identified / Named",   color:"#2ecc71"},
  {bit:2, key:"decomp",   label:"Decompiled to C (unnamed)", color:"#2b6f8a"},
  {bit:0, key:"raw",      label:"Raw Binary",           color:"#414650"},
];
function clsOf(st){ for(const s of STAGES){ if(st & s.bit) return s; }
  return STAGES[STAGES.length-1]; }
const CELL=7, COLS=112, PAD=10, GAPT=16;

let M=[];               /* decoded modules */
let nsFilter=null;

function decode(){
  M = DATA.modules.map(m=>{
    const addrs=m.addrs? m.addrs.split(" "):[];
    return {...m, addrs, st:new Uint8Array(m.st)};
  });
}
function counts(m){
  const c={raw:0,named:0,decomp:0,verified:0,hint:0,recomp:0};
  for(const s of m.st){ if(s&8)c.hint++; if(s&4)c.verified++;
    if(s&16)c.recomp++;
    if(s&1)c.named++; else if(s&2)c.decomp++; else c.raw++; }
  return c;
}

/* ---- header / overview ---- */
function fmt(n){ return n.toLocaleString("en-US"); }
function renderHeader(){
  const t=DATA.totals;
  document.getElementById("game").textContent=DATA.game;
  document.getElementById("stamp").textContent=DATA.generated;
  const pct=(x)=>(100*x/Math.max(t.functions,1)).toFixed(1)+"%";
  document.getElementById("stats").innerHTML=
    `<span class="chip">modules <b>${t.modules}</b></span>`+
    `<span class="chip">functions <b>${fmt(t.functions)}</b></span>`+
    `<span class="chip">identified <b>${fmt(t.named)}</b> (${pct(t.named)})</span>`+
    `<span class="chip">decompiled <b>${fmt(t.decompiled)}</b> (${pct(t.decompiled)})</span>`+
    `<span class="chip">recomp-covered <b>${fmt(t.recomp)}</b> (${pct(t.recomp)})</span>`+
    `<span class="chip">verified <b>${fmt(t.verified)}</b></span>`+
    `<span class="chip">link-ok <b>${t.linkVerified}/${t.modules}</b></span>`+
    (t.hints?`<span class="chip">interpreted <b>${fmt(t.hints)}</b></span>`:"");
  const T={raw:0,named:0,decomp:0,verified:0,hint:0};
  for(const m of M){ const c=counts(m); for(const k in T) T[k]+=c[k]; }
  const bar=document.getElementById("stackbar"), lg=document.getElementById("legend");
  bar.innerHTML=""; lg.innerHTML="";
  for(const s of STAGES){
    const n=T[s.key]||0;
    if(!n && s.key!=="raw" && s.key!=="decomp") continue;
    const w=100*n/Math.max(t.functions,1);
    const i=document.createElement("i");
    i.style.width=w+"%"; i.style.background=s.color; i.title=s.label+": "+fmt(n);
    bar.appendChild(i);
    const l=document.createElement("span");
    l.innerHTML=`<span class="sw" style="background:${s.color}"></span>`+
      `${s.label} <b class="mono">${fmt(n)}</b>`;
    lg.appendChild(l);
  }
}

/* ---- sidebar ---- */
function renderSide(){
  const q=document.getElementById("q").value.toLowerCase();
  const sort=document.getElementById("sort").value;
  let list=M.slice();
  if(q) list=list.filter(m=>m.name.toLowerCase().includes(q));
  list.sort((a,b)=>{
    if(sort==="funcs") return b.n-a.n;
    if(sort==="named") return pct(b)-pct(a);
    if(sort==="unnamed") return (b.n-namedN(b))-(a.n-namedN(a));
    return a.name.localeCompare(b.name);
  });
  const box=document.getElementById("modlist"); box.innerHTML="";
  for(const m of list){
    const c=counts(m), nNamed=c.named, p=100*nNamed/Math.max(m.n,1);
    const row=document.createElement("div"); row.className="modrow";
    row.innerHTML=`<span class="nm mono">${esc(m.name)}</span>`+
      `<span class="pct mono">${p.toFixed(1)}% · ${fmt(m.n)}</span>`+
      `<div class="bar"><i style="width:${100*c.verified/m.n}%;`+
      `background:${colorOf("verified")}"></i>`+
      `<i style="width:${100*nNamed/m.n}%;background:${colorOf("named")}"></i>`+
      `<i style="width:${100*c.decomp/m.n}%;background:${colorOf("decomp")}"></i>`+
      `<i style="width:${100*c.raw/m.n}%;background:${colorOf("raw")}"></i></div>`;
    row.onclick=()=>document.getElementById("mod-"+cssId(m.name))
      .scrollIntoView({behavior:"smooth",block:"start"});
    box.appendChild(row);
  }
}
function namedN(m){ let n=0; for(const s of m.st) if(s&1)n++; return n; }
function pct(m){ return 100*namedN(m)/Math.max(m.n,1); }
function colorOf(k){ return STAGES.find(s=>s.key===k).color; }
function esc(s){ return s.replace(/&/g,"&amp;").replace(/</g,"&lt;"); }
function cssId(s){ return "mod-"+s.replace(/[^A-Za-z0-9_-]/g,"_"); }

/* ---- namespaces ---- */
function renderNs(){
  const box=document.getElementById("nslist"); box.innerHTML="";
  const top=DATA.namespaces.slice(0,30);
  if(!top.length){ box.innerHTML=
    `<span class="chip">no mangled-namespace symbols found</span>`; return; }
  for(const rec of top){
    const chip=document.createElement("span"); chip.className="nschip mono";
    chip.textContent=`${rec.ns} · ${rec.count}`;
    if(nsFilter===rec.ns) chip.classList.add("sel");
    chip.onclick=()=>{ nsFilter=(nsFilter===rec.ns)?null:rec.ns;
      renderNs(); rerenderCanvases(); };
    box.appendChild(chip);
  }
  const flt=document.getElementById("nsfilter");
  if(nsFilter){ flt.style.display="block";
    flt.innerHTML=`namespace filter: <b class="mono">${esc(nsFilter)}</b> — `+
      `non-matching cells dimmed · <a href="#" onclick="nsFilter=null;`+
      `renderNs();rerenderCanvases();return false">clear</a>`;
  } else flt.style.display="none";
}
function nsOfCell(m,i){
  const id=m.ns[String(i)];
  return id===undefined?null:DATA.nsIndex[id];
}

/* ---- module sections + canvas ---- */
const rendered=new Set();
function renderMods(){
  const host=document.getElementById("mods"); host.innerHTML="";
  rendered.clear();
  for(const m of M){
    const c=counts(m);
    const sec=document.createElement("section");
    sec.className="mod"; sec.id=cssId(m.name);
    const head=document.createElement("header");
    head.innerHTML=`<h3 class="mono">${esc(m.name)}</h3>`+
      `<span class="meta"><b>${fmt(m.n)}</b> fns</span>`+
      `<span class="meta">identified <b>${fmt(c.named)}</b></span>`+
      `<span class="meta">decompiled <b>${fmt(c.decomp+c.named)}</b></span>`+
      `<span class="meta">recomp <b>${fmt(c.recomp)}</b></span>`+
      (c.verified?`<span class="meta">verified <b>${fmt(c.verified)}</b></span>`:"")+
      (c.hint?`<span class="meta">interpreted <b style="color:#e74c3c">`+
        `${fmt(c.hint)}</b></span>`:"")+
      `<span class="meta">${m.linkOk?"🔗 link-verified":"link —"}</span>`+
      `<span class="spacer"></span>`+
      `<button class="btn" data-svg>SVG</button>`+
      `<button class="btn" data-png>PNG</button>`;
    const cv=document.createElement("canvas");
    sec.appendChild(cv);
    if(m.imports.length){
      const im=document.createElement("div"); im.className="imports";
      im.innerHTML=`<span class="meta" style="align-self:center">imports:</span>`;
      for(const dep of m.imports.slice(0,40)){
        const d=document.createElement("span"); d.className="imp mono";
        d.textContent=dep;
        d.onclick=()=>{ const t=document.getElementById(cssId(dep));
          if(t) t.scrollIntoView({behavior:"smooth",block:"start"}); };
        im.appendChild(d);
      }
      if(m.imports.length>40){ const more=document.createElement("span");
        more.className="meta"; more.textContent=`+${m.imports.length-40} more`;
        im.appendChild(more); }
      const impBy=m.importers;
      if(impBy.length){
        const lbl=document.createElement("span"); lbl.className="meta";
        lbl.style.alignSelf="center"; lbl.textContent="imported by:";
        im.appendChild(lbl);
        for(const dep of impBy.slice(0,20)){
          const d=document.createElement("span"); d.className="imp mono";
          d.textContent=dep;
          d.onclick=()=>{ const t=document.getElementById(cssId(dep));
            if(t) t.scrollIntoView({behavior:"smooth",block:"start"}); };
          im.appendChild(d);
        }
      }
      sec.appendChild(im);
    }
    host.appendChild(sec);

    const layout={cv,sec,m};
    head.querySelector("[data-svg]").onclick=()=>exportModuleSvg(m);
    head.querySelector("[data-png]").onclick=()=>exportModulePng(m);
    cv.addEventListener("mousemove",e=>onMove(e,layout));
    cv.addEventListener("mouseleave",hideTip);

    const io=new IntersectionObserver(es=>{
      for(const e of es) if(e.isIntersecting){ drawModule(layout); }
    },{rootMargin:"300px"});
    io.observe(cv);
  }
}
function layoutOf(m, availW){
  let cols=COLS;
  if(availW){ cols=Math.floor((availW-2*PAD)/CELL); }
  cols=Math.max(1,Math.min(cols,m.n));
  const rows=Math.ceil(m.n/cols);
  const w=PAD*2+cols*CELL, h=PAD*2+rows*CELL+GAPT;
  return {cols,rows,w,h};
}
function cellColor(m,i){
  const base=clsOf(m.st[i]).color;
  if(nsFilter && nsOfCell(m,i)!==nsFilter){
    return shade(base,0.22);          /* dimmed */
  }
  return base;
}
function shade(hex,f){
  const n=parseInt(hex.slice(1),16);
  const r=(n>>16)&255,g=(n>>8)&255,b=n&255;
  const mix=c=>Math.round(c*f+18*(1-f));
  return `rgb(${mix(r)},${mix(g)},${mix(b)})`;
}
function drawModule({cv,sec,m}){
  const avail=cv.parentNode ? cv.parentNode.clientWidth-30 : 0;
  const {cols,rows,w,h}=layoutOf(m, avail||undefined);
  const dpr=window.devicePixelRatio||1;
  cv.width=w*dpr; cv.height=h*dpr;
  cv.style.width=w+"px"; cv.style.height=h+"px";
  const g=cv.getContext("2d"); g.scale(dpr,dpr);
  g.fillStyle="#14161a"; g.fillRect(0,0,w,h);
  for(let i=0;i<m.n;i++){
    const x=PAD+(i%cols)*CELL, y=PAD+GAPT+Math.floor(i/cols)*CELL;
    g.fillStyle=cellColor(m,i);
    g.fillRect(x,y,CELL-1,CELL-1);
  }
  rendered.add(m.name);
}
function rerenderCanvases(){
  for(const {cv,sec,m} of allLayouts()) if(rendered.has(m.name)) drawModule({cv,sec,m});
  renderSide();
}
function allLayouts(){
  const out=[];
  for(const sec of document.querySelectorAll("section.mod")){
    const m=M.find(x=>cssId(x.name)===sec.id);
    if(m) out.push({cv:sec.querySelector("canvas"),sec,m});
  }
  return out;
}

/* ---- tooltip ---- */
const tip=document.getElementById("tooltip");
function onMove(e,{cv,m}){
  const r=cv.getBoundingClientRect();
  const {cols}=layoutOf(m, r.width);
  const x=Math.floor((e.clientX-r.left-PAD)/CELL);
  const y=Math.floor((e.clientY-r.top-PAD-GAPT)/CELL);
  if(x<0||x>=cols||y<0) return hideTip();
  const i=y*cols+x;
  if(i>=m.n) return hideTip();
  const addr=parseInt(m.addrs[i],16);
  const nm=m.names[String(i)];
  const st=clsOf(m.st[i]);
  const ns=nsOfCell(m,i);
  tip.style.display="block";
  tip.innerHTML=`<div class="t1 mono">${esc(m.name)} · f_${addr.toString(16)}`+
    `</div>`+
    (nm?`<div class="mono">${esc(nm)}</div>`:"")+
    (ns?`<div class="t2 mono">ns ${esc(ns)}</div>`:"")+
    `<div class="t2">${st.label}${m.st[i]&16?" · recomp-covered":""}</div>`;
  const tx=Math.min(e.clientX+14,window.innerWidth-tip.offsetWidth-8);
  const ty=Math.min(e.clientY+14,window.innerHeight-tip.offsetHeight-8);
  tip.style.left=tx+"px"; tip.style.top=ty+"px";
}
function hideTip(){ tip.style.display="none"; }

/* ---- exports ---- */
function download(name,blob){
  const a=document.createElement("a");
  a.href=URL.createObjectURL(blob); a.download=name;
  document.body.appendChild(a); a.click();
  setTimeout(()=>{URL.revokeObjectURL(a.href);a.remove();},400);
}
function stamp(){
  const d=new Date(DATA.generated.replace(" ","T"));
  const p=n=>String(n).padStart(2,"0");
  return `${d.getFullYear()}${p(d.getMonth()+1)}${p(d.getDate())}`+
    `-${p(d.getHours())}${p(d.getMinutes())}`;
}
function svgEsc(s){ return s.replace(/&/g,"&amp;").replace(/</g,"&lt;"); }

function moduleSvg(m,cell,withHeader=true){
  const cols=Math.min(COLS,m.n), rows=Math.ceil(m.n/cols);
  const pad=cell===CELL?PAD:12;
  const headH=withHeader?26:0;
  const w=pad*2+cols*cell, h=pad*2+rows*cell+headH+ (withHeader?10:0);
  const c=counts(m);
  const out=[`<svg xmlns="http://www.w3.org/2000/svg" width="${w}" height="${h}" viewBox="0 0 ${w} ${h}">`,
    `<rect width="100%" height="100%" fill="#14161a"/>`];
  if(withHeader){
    out.push(`<text x="${pad}" y="${pad+8}" fill="#e8e8e8" font-family="monospace" font-size="12">`+
      `${svgEsc(m.name)} — ${m.n} functions · identified ${c.named}`+
      `${c.verified?` · verified ${c.verified}`:""}${m.linkOk?" · link-verified":""}</text>`);
  }
  for(let i=0;i<m.n;i++){
    const x=pad+(i%cols)*cell, y=pad+headH+(Math.floor(i/cols))*cell;
    const nm=m.names[String(i)];
    const t=`${svgEsc(m.name)} f_${parseInt(m.addrs[i],16).toString(16)}`+
      (nm?` — ${svgEsc(nm)}`:"")+` [${clsOf(m.st[i]).label}]`;
    out.push(`<rect x="${x}" y="${y}" width="${cell-1}" height="${cell-1}" fill="${cellColor(m,i)}"><title>${t}</title></rect>`);
  }
  out.push("</svg>");
  return out.join("\n");
}

function allSvg(){
  const cell=6, cols=COLS, pad=12, headH=30;
  const t=DATA.totals;
  const parts=[];
  const modBlocks=[];
  let y=72;
  for(const m of M){
    const rows=Math.ceil(m.n/cols);
    modBlocks.push({m,y});
    y+=headH+rows*cell+18;
  }
  const w=pad*2+cols*cell, h=y;
  parts.push(`<svg xmlns="http://www.w3.org/2000/svg" width="${w}" height="${h}" viewBox="0 0 ${w} ${h}">`,
    `<rect width="100%" height="100%" fill="#0f1115"/>`,
    `<text x="${pad}" y="24" fill="#e8e8e8" font-family="monospace" font-size="15">PokeXYZ-RDComp heatmap — ${svgEsc(DATA.game)}</text>`,
    `<text x="${pad}" y="44" fill="#9aa0aa" font-family="monospace" font-size="11">${DATA.generated} · ${fmt(t.functions)} functions · identified ${fmt(t.named)} · verified ${fmt(t.verified)} · link-ok ${t.linkVerified}/${t.modules}</text>`);
  let lx=pad;
  for(const s of STAGES){
    parts.push(`<rect x="${lx}" y="52" width="9" height="9" fill="${s.color}"/>`,
      `<text x="${lx+13}" y="60" fill="#9aa0aa" font-family="monospace" font-size="10">${s.label}</text>`);
    lx+=13+s.label.length*6.2+18;
  }
  for(const {m,y:my} of modBlocks){
    const c=counts(m);
    parts.push(`<text x="${pad}" y="${my+14}" fill="#e8e8e8" font-family="monospace" font-size="11">${svgEsc(m.name)} — ${m.n} fns · identified ${c.named}${m.linkOk?" · link-verified":""}</text>`);
    for(let i=0;i<m.n;i++){
      const x=pad+(i%cols)*cell, yy=my+headH+Math.floor(i/cols)*cell;
      const nm=m.names[String(i)];
      const t=`${svgEsc(m.name)} f_${parseInt(m.addrs[i],16).toString(16)}`+
        (nm?` — ${svgEsc(nm)}`:"")+` [${clsOf(m.st[i]).label}]`;
      parts.push(`<rect x="${x}" y="${yy}" width="${cell-1}" height="${cell-1}" fill="${cellColor(m,i)}"><title>${t}</title></rect>`);
    }
  }
  parts.push("</svg>");
  return {svg:parts.join("\n"), w, h: y};
}

function exportModuleSvg(m){
  download(`heatmap-${m.name.replace(/[^\w.-]/g,"_")}-${stamp()}.svg`,
    new Blob([moduleSvg(m,7)],{type:"image/svg+xml"}));
}
function exportModulePng(m){
  const scale=3, cell=7;
  const svg=moduleSvg(m,cell);
  rasterize(svg,scale).then(canvas=>
    canvas.toBlob(b=>download(
      `heatmap-${m.name.replace(/[^\w.-]/g,"_")}-${stamp()}.png`,b)));
}
function exportAllSvg(){
  download(`pokexyz-heatmap-${stamp()}.svg`,
    new Blob([allSvg().svg],{type:"image/svg+xml"}));
}
function exportAllPng(){
  const {svg,w,h}=allSvg();
  const maxArea=2.0e8;
  let scale=2;
  if(w*h*scale*scale>maxArea) scale=Math.sqrt(maxArea/(w*h));
  rasterize(svg,scale).then(canvas=>
    canvas.toBlob(b=>download(`pokexyz-heatmap-${stamp()}.png`,b)));
}
function rasterize(svgStr,scale){
  return new Promise((res,rej)=>{
    const blob=new Blob([svgStr],{type:"image/svg+xml;charset=utf-8"});
    const url=URL.createObjectURL(blob);
    const img=new Image();
    img.onload=()=>{
      const cv=document.createElement("canvas");
      cv.width=Math.round(img.width*scale);
      cv.height=Math.round(img.height*scale);
      const g=cv.getContext("2d");
      g.fillStyle="#0f1115"; g.fillRect(0,0,cv.width,cv.height);
      g.drawImage(img,0,0,cv.width,cv.height);
      URL.revokeObjectURL(url); res(cv);
    };
    img.onerror=rej;
    img.src=url;
  });
}
document.getElementById("expSvgAll").onclick=exportAllSvg;
document.getElementById("expPngAll").onclick=exportAllPng;

/* ---- live polling ---- */
function goLive(){
  const el=document.getElementById("live");
  if(!location.protocol.startsWith("http")){
    document.getElementById("livetxt").textContent=
      "static — serve with --serve";
    return;
  }
  el.classList.add("on");
  document.getElementById("livetxt").textContent="live";
  setInterval(async()=>{
    try{
      const r=await fetch(`heatmap_data.json?v=${Date.now()}`,{cache:"no-store"});
      if(!r.ok) return;
      const d=await r.json();
      if(d.generated!==DATA.generated){
        Object.assign(DATA,d);
        decode(); renderHeader(); renderSide(); renderNs(); renderMods();
      } else {
        document.getElementById("stamp").textContent=d.generated;
      }
    }catch(_e){/* transient */}
  },3000);
}

/* ---- boot ---- */
document.getElementById("q").addEventListener("input",renderSide);
document.getElementById("sort").addEventListener("change",renderSide);
let rszT=null;
window.addEventListener("resize",()=>{ clearTimeout(rszT);
  rszT=setTimeout(rerenderCanvases,150); });
decode(); renderHeader(); renderSide(); renderNs(); renderMods(); goLive();
</script>
</body>
</html>
"""


def build(args):
    data = collect(args)
    out_dir = args.out
    os.makedirs(out_dir, exist_ok=True)
    payload = json.dumps(data, separators=(",", ":"))
    with open(os.path.join(out_dir, "heatmap_data.json"), "w") as f:
        f.write(payload)
    page = HTML_TEMPLATE.replace("__DATA_JSON__", payload)
    with open(os.path.join(out_dir, "heatmap.html"), "w") as f:
        f.write(page)
    render_summary(data, out_dir)
    t = data["totals"]
    print(f"wrote {out_dir}/heatmap.html + heatmap_data.json + summary.md")
    print(f"  {t['modules']} modules · {t['functions']:,} functions · "
          f"{t['named']:,} identified ({100.0 * t['named'] / max(t['functions'], 1):.1f}%) · "
          f"{t['decompiled']:,} decompiled · {t['recomp']:,} recomp-covered · "
          f"{t['verified']} verified · link-ok {t['linkVerified']}/{t['modules']}")
    return data


def input_fingerprint(args):
    src = args.source_root
    paths = [os.path.join(args.recomp or os.path.join(src, "recomp"),
                          "entries.c")]
    paths += glob.glob(os.path.join(args.recomp or os.path.join(src, "recomp"),
                                    "*_functions.h"))
    for d in (args.symbols or os.path.join(src, "pipeline", "symbols"),
              args.cro or os.path.join(src, "cro_extracted")):
        paths += glob.glob(os.path.join(d, "*", "symbols.tsv"))
        paths += glob.glob(os.path.join(d, "*.csv"))
        paths += glob.glob(os.path.join(d, "*", "imports.tsv"))
    fp = []
    for p in paths:
        try:
            st = os.stat(p)
            fp.append(f"{p}:{st.st_mtime_ns}:{st.st_size}")
        except OSError:
            pass
    return hashlib.sha256("|".join(fp).encode()).hexdigest()


def serve(out_dir, port, watch_args=None):
    import http.server
    import functools
    import threading

    if watch_args is not None:
        def _watch():
            last = input_fingerprint(watch_args)
            while True:
                time.sleep(2)
                cur = input_fingerprint(watch_args)
                if cur != last:
                    last = cur
                    try:
                        build(watch_args)
                    except Exception as e:  # keep serving on regen errors
                        print(f"watch: rebuild failed: {e}", file=sys.stderr)
        threading.Thread(target=_watch, daemon=True).start()

    handler = functools.partial(
        http.server.SimpleHTTPRequestHandler, directory=out_dir)
    url = f"http://127.0.0.1:{port}/heatmap.html"
    print(f"serving {out_dir} -> {url} (Ctrl-C to stop)")
    http.server.ThreadingHTTPServer(("127.0.0.1", port), handler).serve_forever()


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--source-root", default=".Data/Source_Extracted")
    ap.add_argument("--recomp", help="3dsrecomp port dir "
                    "(default <src>/recomp)")
    ap.add_argument("--symbols", help="pipeline symbols dir "
                    "(default <src>/pipeline/symbols)")
    ap.add_argument("--cro", help="cro_extracted root (default <src>/cro_extracted)")
    ap.add_argument("--decomp", help="decomp corpus root (default <src>/decomp)")
    ap.add_argument("--pipeline", help="matching-decomp pipeline dir "
                    "(default <src>/pipeline)")
    ap.add_argument("--hints", help="Zakuro interpreter hints file "
                    "(addresses the AOT recomp didn't cover)")
    ap.add_argument("--verified", help="JSON list of {module, addr} records "
                    "verified natively (3dsrecomp verify / objdiff export)")
    ap.add_argument("--out", default=".Data/Source_Extracted/heatmap_gui")
    ap.add_argument("--watch", action="store_true",
                    help="rebuild when inputs change")
    ap.add_argument("--serve", type=int, metavar="PORT", nargs="?", const=8765,
                    help="serve the output dir over http (live page)")
    args = ap.parse_args()

    if args.serve is not None:
        build(args)  # ensure fresh output before serving
        serve(args.out, args.serve, watch_args=args if args.watch else None)
        return
    if args.watch:
        last = input_fingerprint(args)
        build(args)
        while True:
            time.sleep(2)
            cur = input_fingerprint(args)
            if cur != last:
                last = cur
                try:
                    build(args)
                except Exception as e:
                    print(f"watch: rebuild failed: {e}", file=sys.stderr)
    else:
        build(args)


if __name__ == "__main__":
    main()
