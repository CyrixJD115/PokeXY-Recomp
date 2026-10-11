"""Gather per-function status from every Source_Extracted input.

Status bits per function (see page.py for the visual ladder):
  1 identified (real symbol)   2 decompiled (Ghidra C)
  4 natively verified          8 interpreter hints overlay
  16 recomp-covered (3dsrecomp port start; stats only)

The function inventory is the UNION of every tool's function starts —
3dsrecomp merges some functions Ghidra splits and vice versa, so no
single source is authoritative.

Coordinates: Main (functions.h) uses absolute 0x100000-based addresses,
matching code.bin.csv and static.crs load_addr. CRO modules
(m<NNN>_functions.h) use module-relative text offsets (0x180-based),
matching the shifted <Dll>.cro.csv rows and the CRO symbols.tsv `offset`
column; cro_c decomp filenames are load-address based (0x100000+offset)
and are rebased by -0x100000 before matching.
"""
import csv
import glob
import hashlib
import os
import re
import time

MOD_RE = re.compile(r'\{"([^"]+)", &m(\d+)_base, (0x[0-9A-Fa-f]+)u, (\d+),')
FN_RE = re.compile(r'void (?:m\d+_)?f_([0-9A-Fa-f]+)\(Context')

# Ghidra synthetic name families — these are NOT "identified" functions.
SYNTHETIC = ("FUN_", "thunk_FUN_", "DAT_", "UNK_", "LAB_", "SUB_",
             "switchD_", "switchdataD_", "module_entry", "module_")


class Sources:
    """Resolved paths for every input the collector reads."""

    def __init__(self, args):
        src = args.source_root
        self.recomp = args.recomp or os.path.join(src, "recomp")
        self.symbols = args.symbols or os.path.join(src, "pipeline",
                                                    "symbols")
        self.cro = args.cro or os.path.join(src, "cro_extracted")
        self.decomp = args.decomp or os.path.join(src, "decomp")
        self.pipeline = args.pipeline or os.path.join(src, "pipeline")

    def fingerprint_inputs(self):
        """(path, mtime, size) hash of everything collect() reads — the
        --watch loop regenerates when this changes."""
        paths = [os.path.join(self.recomp, "entries.c")]
        paths += glob.glob(os.path.join(self.recomp, "*_functions.h"))
        for d in (self.symbols, self.cro):
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
    """Optional JSON list of {module, addr} records verified natively."""
    import json
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
    orig = resolve_ci(os.path.join(pipeline_dir, "orig"), module_file)
    out = resolve_ci(os.path.join(pipeline_dir, "out"), module_file)
    try:
        if os.path.getsize(orig) != os.path.getsize(out):
            return False
        return sha256_of(orig) == sha256_of(out)
    except (FileNotFoundError, OSError):
        return False


def collect(args, rom=None):
    """Build the whole dashboard payload dict from Sources + optional ROM
    identity (from romdetect.identify / scan)."""
    src = Sources(args)
    mods = parse_module_map(os.path.join(src.recomp, "entries.c"))
    hints = parse_hints(args.hints)
    verified = parse_verified(args.verified)

    # Main executable: absolute addresses; static.crs = its export table.
    plan = [("Main (code.bin)", "code.bin", 0,
             os.path.join(src.recomp, "functions.h"),
             os.path.join(src.symbols, "code.bin.csv"),
             os.path.join(src.cro, "static", "symbols.tsv"),
             os.path.join(src.decomp, "code_c"), 0)]
    # CROs: m<NNN>_functions.h + <Dll>.cro.csv + exports, module-relative.
    for mid, info in sorted(mods.items()):
        h = os.path.join(src.recomp, f"m{mid:03d}_functions.h")
        if os.path.exists(h):
            plan.append((info["name"], info["name"], mid, h,
                         resolve_ci(src.symbols, f"{info['name']}.cro.csv"),
                         os.path.join(src.cro, info["name"], "symbols.tsv"),
                         resolve_ci(os.path.join(src.decomp, "cro_c"),
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

        funcs = sorted(port | gh_starts | decomp_set | set(names) | ver)

        imports = parse_imports(resolve_ci(
            os.path.join(src.cro, "static" if mid == 0 else name),
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
            src.pipeline,
            modfile if mid == 0 else os.path.basename(
                resolve_ci(os.path.join(src.pipeline, "orig"),
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

    # Game identity: explicit --rom wins, else the default X base dump.
    if rom is None:
        from . import romdetect
        rom = romdetect.default_game_rom(args._repo_root)
    if rom and rom.get("recognized"):
        game = f"{rom['game']} · {rom['title_id']} ({rom['kind']})"
        if rom.get("game") != "Pokémon X":
            game += " — data basis is the Pokémon X port"
    elif rom:
        game = f"unrecognized title {rom['title_id']} — data basis is X"
    else:
        game = "Pokémon X (recomp basis: title 0004000000055D00)"

    return {
        "generated": time.strftime("%Y-%m-%d %H:%M:%S"),
        "game": game,
        "rom": ({"title_id": rom["title_id"], "game": rom["game"],
                 "kind": rom["kind"]} if rom else None),
        "totals": {**totals, "modules": len(modules),
                   "linkVerified": link_total},
        "namespaces": namespaces,
        "nsIndex": sorted(ns_index, key=ns_index.get) if ns_index else [],
        "modules": modules,
    }
