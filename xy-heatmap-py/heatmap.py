#!/usr/bin/env python3
"""RDComp heatmap — per-function status map for the Pokémon X/Y recomp/decomp.

Inputs (all produced locally from your own dump; never committed):
  --port     3dsrecomp port dir (code/entries.c + code/*functions.h)
  --symbols  3DS-Decomp-Pipeline symbols dir (code.bin.csv, <Dll>.cro.csv)
  --hints    optional: 3dsrecomp hints file (addresses where Zakuro's
             interpreter ran — i.e. code the recompiler didn't cover)
  --out      output directory (default ./heatmap)

Outputs:
  summary.md     per-module stats: functions, named %, interpreter hits
  <module>.svg   address-ordered function grid; green = has a real name,
                 yellow = Ghidra-default name only (FUN_*), red = interpreted
                 at runtime (hints)

Coordinates: the Main module (functions.h) uses absolute 0x100000-based
addresses, matching code.bin.csv; CRO modules use module-relative text
offsets (0x180-based), matching the shifted <Dll>.cro.csv. Addresses
intersect directly.
"""
import argparse
import html
import os
import re
from collections import defaultdict

MOD_RE = re.compile(r'\{"([^"]+)", &m(\d+)_base, (0x[0-9A-Fa-f]+)u, (\d+),')
FN_RE = re.compile(r'void (?:m\d+_)?f_([0-9A-Fa-f]+)\(Context')


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
    addrs = []
    with open(path) as f:
        for line in f:
            m = FN_RE.search(line)
            if m:
                addrs.append(int(m.group(1), 16))
    return addrs


def parse_symbols(csv_path):
    named = set()
    try:
        import csv as _csv
        with open(csv_path) as f:
            for row in _csv.DictReader(f):
                name = row["Name"].strip('"')
                if not name.startswith(("FUN_", "thunk_FUN_", "DAT_", "UNK_")):
                    try:
                        named.add(int(row["Location"].strip('"'), 16))
                    except ValueError:
                        pass
    except FileNotFoundError:
        pass
    return named


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
                a = int(line, 16)
                addrs.add(a & ~1)  # odd = Thumb
            except ValueError:
                pass
    return addrs


def render_svg(module, funcs, named, hints, out_path):
    cell, cols = 7, 110
    rows = (len(funcs) + cols - 1) // cols
    pad, legend_h = 14, 34
    w, h = cols * cell + 2 * pad, rows * cell + 2 * pad + legend_h
    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" '
        f'viewBox="0 0 {w} {h}">',
        f'<rect width="100%" height="100%" fill="#14161a"/>',
        f'<text x="{pad}" y="{pad + 4}" fill="#e8e8e8" font-family="monospace" '
        f'font-size="13">{html.escape(module)} — {len(funcs)} functions</text>',
    ]
    named_n = sum(1 for a in funcs if a in named)
    hint_n = sum(1 for a in funcs if a in hints)
    y0 = pad + 16
    for i, a in enumerate(funcs):
        x = pad + (i % cols) * cell
        y = y0 + (i // cols) * cell
        if a in hints:
            color = "#e74c3c"  # interpreted at runtime
        elif a in named:
            color = "#2ecc71"  # has a real name
        else:
            color = "#f1c40f"  # Ghidra-default name only
        parts.append(f'<rect x="{x}" y="{y}" width="{cell-1}" height="{cell-1}" '
                     f'fill="{color}"><title>{module} f_{a:08X}</title></rect>')
    ly = y0 + rows * cell + 14
    for i, (c, label) in enumerate([("#2ecc71", f"named {named_n}"),
                                    ("#f1c40f", f"unnamed {len(funcs)-named_n}"),
                                    ("#e74c3c", f"interpreted {hint_n}")]):
        x = pad + i * 190
        parts.append(f'<rect x="{x}" y="{ly}" width="10" height="10" fill="{c}"/>')
        parts.append(f'<text x="{x+16}" y="{ly+9}" fill="#aaaaaa" '
                     f'font-family="monospace" font-size="12">{label}</text>')
    parts.append("</svg>")
    with open(out_path, "w") as f:
        f.write("\n".join(parts))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--port", required=True, help="3dsrecomp port dir")
    ap.add_argument("--symbols", help="pipeline symbols dir")
    ap.add_argument("--hints", help="3dsrecomp hints file")
    ap.add_argument("--out", default="heatmap")
    args = ap.parse_args()

    code = os.path.join(args.port, "code")
    mods = parse_module_map(os.path.join(code, "entries.c"))
    os.makedirs(args.out, exist_ok=True)
    hints = parse_hints(args.hints)

    # Main executable: functions.h, absolute addresses
    modules = [("Main (code.bin)", os.path.join(code, "functions.h"),
                os.path.join(args.symbols or "", "code.bin.csv"))]
    # CROs: mXXX_functions.h, module-relative addresses
    for mid, info in sorted(mods.items()):
        p = os.path.join(code, f"m{mid:03d}_functions.h")
        if os.path.exists(p):
            modules.append((info["name"], p,
                            os.path.join(args.symbols or "", f"{info['name']}.cro.csv")))

    rows = ["| module | functions | named | named % | interpreted (hints) |",
            "|---|---:|---:|---:|---:|"]
    for name, hpath, csvpath in modules:
        funcs = sorted(set(parse_functions(hpath)))
        if not funcs:
            continue
        named = parse_symbols(csvpath) if csvpath else set()
        pct = 100.0 * len(named & set(funcs)) / len(funcs)
        hits = sum(1 for a in funcs if a in hints)
        rows.append(f"| {name} | {len(funcs)} | {len(named & set(funcs))} | "
                    f"{pct:.1f}% | {hits} |")
        render_svg(name, funcs, named, hints,
                   os.path.join(args.out, f"{name.replace(' ', '_')}.svg"))

    total_f = total_named = total_hint = 0
    for r in rows[2:]:
        c = r.split("|")
        total_f += int(c[2]); total_named += int(c[3]); total_hint += int(c[5])
    with open(os.path.join(args.out, "summary.md"), "w") as f:
        f.write("# RDComp heatmap\n\n")
        f.write("\n".join(rows))
        f.write(f"\n\n**Total: {total_f} functions, {total_named} named "
                f"({100.0*total_named/max(total_f,1):.1f}%), "
                f"{total_hint} interpreter hits.**\n")
    print(f"wrote {args.out}/summary.md + {len(modules)} SVGs")
    print("\n".join(rows[:6]) + "\n…")


if __name__ == "__main__":
    main()
