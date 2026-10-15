"""Static SVG renderers for committed README assets + the shared treemap
layout.

The treemap layout is computed HERE, once, and embedded into the page's
JSON payload — the interactive canvas (page.py) and the SVG renderer
below both consume the same rects, so the two renders can never drift.
Everything drawn is axis-aligned with a tight viewBox: no rotated text,
no clipping. Deterministic: same data, byte-identical SVG.

`heatmap.py readme` writes docs/assets/:
  heatmap-treemap.svg    whole-corpus treemap (every module a box)
  heatmap-progress.svg   tier progress bar + recomp coverage strip
"""

import math

# stage colours — keep in sync with page.py's STAGES
COLORS = {
    "hint": "#e74c3c",
    "armcc": "#9b59b6",
    "verified": "#f0b429",
    "named": "#2ecc71",
    "decomp": "#2b6f8a",
    "raw": "#414650",
}
LABELS = {
    "hint": "Interpreted (hints)",
    "armcc": "ARM Compiler 5 Masked-Matched",
    "verified": "Natively Verified",
    "named": "Identified / Named",
    "decomp": "Decompiled to C (unnamed)",
    "raw": "Raw Binary",
}


def cls_of(st):
    """Highest-achieved stage colour key for a status bitmask."""
    if st & 8:
        return "hint"
    if st & 32:
        return "armcc"
    if st & 4:
        return "verified"
    if st & 1:
        return "named"
    if st & 2:
        return "decomp"
    return "raw"


def counts_of(module):
    c = {"raw": 0, "named": 0, "decomp": 0, "verified": 0, "hint": 0,
         "recomp": 0, "armcc": 0}
    for s in module["st"]:
        if s & 8:
            c["hint"] += 1
        if s & 4:
            c["verified"] += 1
        if s & 16:
            c["recomp"] += 1
        if s & 32:
            c["armcc"] += 1
        if s & 1:
            c["named"] += 1
        elif s & 2:
            c["decomp"] += 1
        else:
            c["raw"] += 1
    return c


def _n(v):
    """Compact number formatting for SVG attributes."""
    s = f"{v:.1f}"
    return s[:-2] if s.endswith(".0") else s


def _esc(s):
    return s.replace("&", "&amp;").replace("<", "&lt;")


# ---------------------------------------------------------------- treemap

def _squarify(items, x, y, w, h, out):
    """Bruls-et-al squarified treemap on integer weights."""
    if not items:
        return
    remaining = sum(it["n"] for it in items)
    row = [items[0]]
    i = 1
    length = min(w, h)

    def worst(r):
        s = sum(it["n"] for it in r)
        mx = max(it["n"] for it in r)
        mn = min(it["n"] for it in r)
        return max(length * length * mx / (s * s),
                   s * s / (length * length * mn))

    while i < len(items):
        if worst(row + [items[i]]) <= worst(row):
            row.append(items[i])
            i += 1
        else:
            break
    s = sum(it["n"] for it in row)
    if w >= h:                                   # vertical strip on the left
        dx = w * s / remaining
        cy = y
        for it in row:
            dh = h * it["n"] / s
            out.append({"name": it["name"], "n": it["n"],
                        "x": x, "y": cy, "w": dx, "h": dh})
            cy += dh
        _squarify(items[i:], x + dx, y, w - dx, h, out)
    else:                                        # horizontal strip on top
        dy = h * s / remaining
        cx = x
        for it in row:
            dw = w * it["n"] / s
            out.append({"name": it["name"], "n": it["n"],
                        "x": cx, "y": y, "w": dw, "h": dy})
            cx += dw
        _squarify(items[i:], x, y + dy, w, h - dy, out)


def treemap_layout(data, width=1000, height=640, pad=2):
    """Module boxes sized by function count + per-box cell grid params.

    Cells fill each box completely: cell_w covers the width, cell_h covers
    the derived row count (clamped 2..8 px, so cells can be slightly
    non-square in huge boxes — still crisp). Embedded into the page JSON
    (data.treemap); both renderers treat the rects as read-only truth.
    """
    items = sorted(data["modules"], key=lambda m: -m["n"])
    rects = []
    _squarify(items, 0, 0, width, height, rects)
    total = sum(m["n"] for m in data["modules"])
    for r in rects:
        strip = 12 if r["h"] >= 26 else 0        # label strip
        aw = max(1.0, r["w"] - 2 * pad)
        ah = max(1.0, r["h"] - strip - pad)
        cell_w = max(2, min(8, int(math.sqrt(aw * ah / max(r["n"], 1)))))
        cols = max(1, int(aw // cell_w))
        rows = math.ceil(r["n"] / cols)
        cell_h = max(2, min(8, int(ah // max(rows, 1))))
        r.update(strip=strip, cell_w=cell_w, cell_h=cell_h, cols=cols,
                 rows=rows)
    return {"width": width, "height": height, "total": total,
            "rects": rects}


def treemap_svg(data, layout=None):
    """The central treemap as a self-contained SVG (row-run merged cells,
    horizontal labels only)."""
    lay = layout or data.get("treemap") or treemap_layout(data)
    w, h = lay["width"], lay["height"]
    out = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{w}" '
        f'height="{h}" viewBox="0 0 {w} {h}">',
        f'<rect width="{w}" height="{h}" fill="#101216"/>',
    ]
    for r in lay["rects"]:
        m = next(m for m in data["modules"] if m["name"] == r["name"])
        x, y, rw, rh = r["x"], r["y"], r["w"], r["h"]
        out.append(f'<rect x="{_n(x)}" y="{_n(y)}" width="{_n(rw)}" '
                   f'height="{_n(rh)}" fill="#181b21" stroke="#23262d" '
                   f'stroke-width="1"/>')
        cell_w, cell_h, cols, strip = (r["cell_w"], r["cell_h"],
                                       r["cols"], r["strip"])
        ox, oy = x + 2, y + strip
        n = m["n"]
        for row in range(r["rows"]):
            in_row = min(cols, n - row * cols)
            run = None
            runs = []
            for j in range(in_row):
                col = COLORS[cls_of(m["st"][row * cols + j])]
                if run and run[0] == col and run[2] == j - 1:
                    run[2] = j
                else:
                    if run:
                        runs.append(run)
                    run = [col, j, j]
            if run:
                runs.append(run)
            ry = oy + row * cell_h
            if ry + cell_h > y + rh:             # never leak out of the box
                break
            for col, j0, j1 in runs:
                rx = ox + j0 * cell_w
                rw_c = (j1 - j0 + 1) * cell_w - 1
                rw_c = min(rw_c, x + rw - rx - 1)
                out.append(f'<rect x="{_n(rx)}" y="{_n(ry)}" '
                           f'width="{_n(rw_c)}" height="{_n(cell_h - 1)}" '
                           f'fill="{col}"/>')
        if strip and rw >= 44:
            name = r["name"]
            max_chars = max(3, int((rw - 10) / 5.6))
            if len(name) > max_chars:
                name = name[:max_chars - 1] + "…"
            out.append(f'<text x="{_n(x + 4)}" y="{_n(y + strip - 3)}" '
                       f'fill="#c9ced6" font-family="monospace" '
                       f'font-size="9">{_esc(name)}</text>')
            if rw >= 110:
                out.append(f'<text x="{_n(x + rw - 4)}" '
                           f'y="{_n(y + strip - 3)}" fill="#79818c" '
                           f'font-family="monospace" font-size="9" '
                           f'text-anchor="end">{r["n"]:,}</text>')
    t = data["totals"]
    pct = 100.0 * t["named"] / max(t["functions"], 1)
    out.append(
        f'<text x="{w - 4}" y="{h - 4}" fill="#79818c" '
        f'font-family="monospace" font-size="10" text-anchor="end">'
        f'{t["functions"]:,} functions · {t["modules"]} modules · '
        f'identified {pct:.1f}% · {data["generated"]}</text>')
    out.append("</svg>")
    return "\n".join(out)


# ------------------------------------------------------------ progress bar

def progress_svg(data, width=880):
    """Stacked tier progress bar + recomp-coverage strip, as SVG.
    Vertical rhythm: title → bar → legend → strip → caption (no overlap)."""
    t = data["totals"]
    total = max(t["functions"], 1)
    tiers = {k: 0 for k in
             ("verified", "named", "decomp", "raw", "hint", "recomp",
              "armcc")}
    for m in data["modules"]:
        for k, v in counts_of(m).items():
            tiers[k] += v
    shown = [k for k in ("armcc", "verified", "named", "decomp", "raw")
             if tiers[k] or k in ("raw", "decomp")]

    title_y, bar_y, bar_h = 20, 32, 26
    legend_y = bar_y + bar_h + 22          # 80
    strip_y = legend_y + 18                # 98
    strip_h = 8
    caption_y = strip_y + strip_h + 20     # 126
    h = caption_y + 12                     # 138
    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" '
        f'height="{h}" viewBox="0 0 {width} {h}">',
        f'<rect width="100%" height="100%" fill="#0f1115" rx="8"/>',
        f'<text x="16" y="{title_y}" fill="#e8e8e8" '
        f'font-family="sans-serif" font-size="13" font-weight="600">'
        f'{_esc(data["game"])} — recomp/decomp progress</text>',
    ]
    x = 16.0
    w = width - 32
    for k in shown:
        seg = w * tiers[k] / total
        if seg <= 0:
            continue
        parts.append(f'<rect x="{_n(x)}" y="{bar_y}" '
                     f'width="{_n(max(seg - 1, 0.5))}" height="{bar_h}" '
                     f'fill="{COLORS[k]}"/>')
        x += seg
    parts.append(f'<rect x="16" y="{bar_y}" width="{_n(w)}" '
                 f'height="{bar_h}" fill="none" stroke="#23262d" rx="3"/>')

    lx = 16.0
    for k in shown:
        if not tiers[k]:
            continue
        pct = 100.0 * tiers[k] / total
        label = f"{LABELS[k]} {tiers[k]:,} ({pct:.1f}%)"
        parts.append(f'<rect x="{_n(lx)}" y="{legend_y - 9}" width="10" '
                     f'height="10" rx="2" fill="{COLORS[k]}"/>')
        parts.append(f'<text x="{_n(lx + 15)}" y="{legend_y}" '
                     f'fill="#9aa0aa" font-family="monospace" '
                     f'font-size="11">{label}</text>')
        lx += 15 + len(label) * 7.0 + 22

    recomp = t.get("recomp", 0)
    parts.append(f'<text x="16" y="{strip_y + 8}" fill="#79818c" '
                 f'font-family="monospace" font-size="9">'
                 f'recomp-covered</text>')
    parts.append(f'<rect x="108" y="{strip_y}" '
                 f'width="{_n(w - 92)}" height="{strip_h}" '
                 f'fill="#232a33" rx="3"/>')
    if recomp:
        parts.append(f'<rect x="108" y="{strip_y}" '
                     f'width="{_n((w - 92) * recomp / total)}" '
                     f'height="{strip_h}" fill="#4fc3f7" rx="3"/>')
    parts.append(f'<text x="16" y="{caption_y}" fill="#79818c" '
                 f'font-family="monospace" font-size="10">'
                 f'{100.0 * recomp / total:.1f}% of functions run from the '
                 f'AOT recompilation · link-ok {t["linkVerified"]}/'
                 f'{t["modules"]} modules · generated {data["generated"]}'
                 f'</text>')
    parts.append("</svg>")
    return "\n".join(parts)


def build_readme_assets(data, out_dir):
    """Write heatmap-treemap.svg + heatmap-progress.svg; return paths."""
    import os
    os.makedirs(out_dir, exist_ok=True)
    treemap = os.path.join(out_dir, "heatmap-treemap.svg")
    progress = os.path.join(out_dir, "heatmap-progress.svg")
    with open(treemap, "w") as f:
        f.write(treemap_svg(data))
    with open(progress, "w") as f:
        f.write(progress_svg(data))
    return treemap, progress
