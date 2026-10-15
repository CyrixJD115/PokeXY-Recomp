"""Pure geometry: Dataset -> Scene.

Layout functions take a dataset and a Config and return a Scene of
primitives — no I/O, no rendering, fully deterministic. Views:

* :func:`treemap_scene`      — one box per module (area = function count),
                               filled with per-function cells in address
                               order, strong box borders, labeled
* :func:`overview_scene`     — 100%-stacked tier-composition bar per module
* :func:`corpus_grid_scene`  — every function of the corpus in one grid,
                               grouped into labeled per-module sections
* :func:`grid_scene`         — one module's functions, address-ordered
* :func:`namespaces_scene`   — symbol-namespace breakdown with module
                               attribution and explanatory labels

All grid-bearing views honour ``min_tier`` (0..3): cells below the tier are
excluded from the grid (headers state the filter).
"""

from __future__ import annotations

import math

from xyheatmap.config import Config
from xyheatmap.dataset import Dataset, Module
from xyheatmap.errors import HeatmapError
from xyheatmap.palette import HINT_COLOR, TIER_COLORS, TIER_LABELS, TIER_LADDER, tier_color
from xyheatmap.scene import Cells, Rect, Scene, Text

# Average glyph-width factor for the sans font we ship with (used only to
# decide truncation; SVG anchoring does the real alignment).
_CHAR_W = 0.58

TIER_NAMES = ["raw", "decompiled", "named", "verified"]


def _fit(s: str, max_w: float, size: float) -> str:
    if max_w <= 0:
        return ""
    if len(s) * _CHAR_W * size <= max_w:
        return s
    max_chars = max(1, int(max_w / (_CHAR_W * size)) - 1)
    return s[:max_chars].rstrip() + "…"


def _pct(n: int, d: int) -> str:
    return f"{(100.0 * n / d):.1f}%" if d else "0.0%"


def _stats_line(ds: Dataset) -> str:
    total = ds.total_cells()
    counts = ds.tier_counts()
    parts = [
        f"{len(ds.modules)} modules",
        f"{total:,} functions",
        f"covered {_pct(ds.covered_count(), total)}",
        f"decompiled {_pct(counts['decompiled'], total)}",
        f"named {_pct(counts['named'], total)}",
        f"verified {_pct(counts['verified'], total)}",
    ]
    relink = ds.relink_summary()
    if relink:
        parts.append(f"relink {relink[0]}/{relink[1]}")
    return " · ".join(parts)


def _legend(scene: Scene, ds: Dataset, cfg: Config, y: float, min_tier: int = 0) -> None:
    counts = ds.tier_counts()
    x = cfg.margin
    for i, tier in enumerate(TIER_LADDER):
        if i < min_tier:
            continue
        label = f"{TIER_LABELS[tier]} {counts[tier]:,}"
        scene.add(Rect(x, y, 11, 11, TIER_COLORS[tier]))
        scene.add(Text(x + 16, y + 9.5, label, cfg.small_size, cfg.muted))
        x += 16 + _CHAR_W * cfg.small_size * len(label) + 20
    hint_label = f"interpreter hits {ds.hint_count():,}"
    scene.add(Rect(x, y, 11, 11, cfg.panel, stroke=HINT_COLOR, stroke_width=1.4))
    scene.add(Text(x + 16, y + 9.5, hint_label, cfg.small_size, cfg.muted))
    scene.add(
        Text(
            cfg.width - cfg.margin,
            y + 9.5,
            "color = highest stage reached",
            cfg.small_size,
            cfg.muted,
            anchor="end",
        )
    )


def _chrome(ds: Dataset, cfg: Config, height: float, subtitle: str, width: float | None = None, title: bool = True) -> Scene:
    width = width if width is not None else cfg.width
    scene = Scene(width, height, cfg.bg, cfg.font_family)
    if title:
        scene.add(
            Text(cfg.margin, cfg.margin + 14, ds.title, cfg.title_size, cfg.text, weight="bold")
        )
        scene.add(Text(cfg.margin, cfg.margin + 34, subtitle, cfg.small_size, cfg.muted))
    else:
        scene.add(Text(cfg.margin, cfg.margin + 20, subtitle, cfg.label_size, cfg.muted))
    # compact chrome keeps only the subtitle; the UI header has the stats.
    # Narrow scenes (module grids) would overlap the title with the stats
    # line, so narrow chrome drops it — the subtitle carries module stats.
    if title and width >= 1150:
        scene.add(
            Text(width - cfg.margin, cfg.margin + 14, _stats_line(ds), cfg.stats_size, cfg.muted, anchor="end")
        )
    scene.add(
        Rect(cfg.margin, cfg.header_h - 8, width - 2 * cfg.margin, 1, cfg.line)
    )
    return scene


def _tip_for(mod: Module, min_tier: int = 0) -> str:
    c = mod.tier_counts()
    lines = [
        f"{mod.name} — {len(mod.cells):,} functions",
        f"raw {c['raw']:,} · decompiled {c['decompiled']:,} · "
        f"named {c['named']:,} · verified {c['verified']:,}",
        f"recomp-covered {_pct(mod.covered_count(), len(mod.cells))}",
    ]
    if mod.imports:
        shown = ", ".join(mod.imports[:4]) + (f" +{len(mod.imports) - 4} more" if len(mod.imports) > 4 else "")
        lines.append(f"imports from: {shown}")
    if mod.relink_ok is not None:
        lines.append("relink: byte-identical" if mod.relink_ok else "relink: MISMATCH")
    if min_tier:
        lines.append(f"(showing {TIER_NAMES[min_tier]}+ only)")
    return "\n".join(lines)


def _filter_cells(mod: Module, min_tier: int):
    """(tiers tuple, base_addr, stride) honouring the tier filter."""
    cells = mod.cells
    if min_tier:
        cells = [c for c in cells if c.tier >= min_tier]
    tiers = tuple(c.tier for c in cells)
    base = cells[0].addr if cells else 0
    stride = min((b.addr - a.addr for a, b in zip(cells, cells[1:])), default=4) if len(cells) > 1 else 4
    stride = max(1, min(stride, 64))
    return tiers, base, stride


# ------------------------------------------------------------------ treemap


def _squarify(areas: list[float], x: float, y: float, w: float, h: float) -> list[tuple[float, float, float, float]]:
    """Squarified treemap (Bruls et al.): descending areas in, rects out.

    ``areas`` must sum to ``w * h``. Deterministic: row growth only accepts a
    candidate that does not worsen the worst aspect ratio.
    """
    out: list[tuple[float, float, float, float]] = []
    i, n = 0, len(areas)

    def worst(row: list[float], short: float) -> float:
        s = sum(row)
        mx, mn = max(row), min(row)
        return max((short * short * mx) / (s * s), (s * s) / (short * short * mn))

    while i < n:
        short = min(w, h)
        row = [areas[i]]
        j = i + 1
        while j < n:
            cand = row + [areas[j]]
            if worst(cand, short) <= worst(row, short):
                row = cand
                j += 1
            else:
                break
        s = sum(row)
        if w >= h:
            rw = s / h
            cy = y
            for a in row:
                ch = a / rw
                out.append((x, cy, rw, ch))
                cy += ch
            x += rw
            w -= rw
        else:
            rh = s / w
            cx = x
            for a in row:
                cw = a / rh
                out.append((cx, y, cw, rh))
                cx += cw
            y += rh
            h -= rh
        i = j
    return out


def _major_box(scene: Scene, mod: Module, x, y, w, h, cfg: Config, min_tier: int) -> None:
    """One squarified treemap box: label + as many function cells as fit,
    rows computed against the label band so cells never cross the border."""
    tiers, base, astride = _filter_cells(mod, min_tier)
    cs = cfg.treemap_cell
    pitch = cs + 1.0
    label_h = 15.0 if (w > 60 and h > 40) else 0.0
    cols = max(1, int((w - 8) // pitch))
    rows = max(1, int((h - label_h - 6) // pitch))
    capacity = cols * rows
    scene.add(Rect(x, y, w, h, cfg.panel))
    scene.add(Cells(
        x + 4, y + label_h + 3, cs, cols, tiers[:capacity],
        gap=1.0, addr_stride=astride, base_addr=base, module=mod.name,
        tip=_tip_for(mod, min_tier), attrs=(("data-module", mod.name),),
    ))
    more = len(tiers) - capacity
    if label_h:
        count = ""
        if w > 130:
            count = f"{len(mod.cells):,}"
            if more > 0:
                count += f" (+{more:,})"
        count_w = _CHAR_W * cfg.small_size * len(count) + 10
        name = _fit(mod.name, w - 10 - count_w, cfg.label_size)
        scene.add(Text(x + 5, y + 11, name, cfg.label_size, cfg.text, weight="bold"))
        if count:
            scene.add(Text(x + w - 5, y + 11, count, cfg.small_size, cfg.muted, weight="bold", anchor="end"))
    scene.add(Rect(x, y, w, h, "#00000000", stroke=cfg.border, stroke_width=1.5,
                   tip=_tip_for(mod, min_tier), attrs=(("data-module", mod.name),)))


def _minor_card(scene: Scene, mod: Module, x, y, bw, bh, cols, rows, cfg: Config, min_tier: int) -> None:
    """A fixed-size card for a small module: label + ALL of its cells.

    Card geometry is derived from the minor threshold, so every card has
    enough slots — no truncation, no slivers.
    """
    tiers, base, astride = _filter_cells(mod, min_tier)
    cs = cfg.treemap_cell
    capacity = cols * rows
    scene.add(Rect(x, y, bw, bh, cfg.panel))
    scene.add(Cells(
        x + 4, y + 17, cs, cols, tiers[:capacity],
        gap=1.0, addr_stride=astride, base_addr=base, module=mod.name,
        tip=_tip_for(mod, min_tier), attrs=(("data-module", mod.name),),
    ))
    more = len(tiers) - capacity
    count = f"{len(mod.cells):,}" + (f" (+{more:,})" if more > 0 else "")
    count_w = _CHAR_W * cfg.small_size * len(count) + 10
    scene.add(Text(x + 5, y + 12, _fit(mod.name, bw - 10 - count_w, cfg.label_size), cfg.label_size, cfg.text, weight="bold"))
    scene.add(Text(x + bw - 5, y + 12, count, cfg.small_size, cfg.muted, weight="bold", anchor="end"))
    scene.add(Rect(x, y, bw, bh, "#00000000", stroke=cfg.border, stroke_width=1.5,
                   tip=_tip_for(mod, min_tier), attrs=(("data-module", mod.name),)))


def treemap_scene(ds: Dataset, cfg: Config, min_tier: int = 0, minor_max: int | None = None) -> Scene:
    """Hybrid treemap.

    * Major modules (more than ``minor_max`` functions) share a squarified
      treemap where box area = function count, filled with per-function
      cells in address order.
    * Small modules would collapse into unreadable slivers there, so they
      are promoted into fixed-size cards below — each card sized to hold
      **all** of its cells, labeled and counted. Every module is therefore
      always readable, and no card ever truncates.
    """
    minor_max = cfg.treemap_minor_max if minor_max is None else max(0, minor_max)
    content_w = cfg.width - 2 * cfg.margin
    major_h = cfg.treemap_height
    pitch = cfg.treemap_cell + 1.0

    mods = sorted(ds.modules, key=lambda m: (-len(m.cells), m.name))
    minors = [m for m in mods if len(m.cells) <= minor_max]
    majors = [m for m in mods if len(m.cells) > minor_max]

    # minor card geometry: guaranteed capacity >= minor_max cells
    bw = cfg.treemap_minor_box_w
    card_cols = max(1, int((bw - 8) // pitch))
    card_rows = max(1, math.ceil(minor_max / card_cols))
    bh = 17 + card_rows * pitch + 3
    per_row = max(1, int(content_w // (bw + 6)))
    band_rows = math.ceil(len(minors) / per_row) if minors else 0
    band_h = band_rows * (bh + 6) + (18 if minors else 0)

    height = cfg.header_h + major_h + band_h + cfg.legend_h + cfg.margin
    subtitle = (
        "treemap hybrid — big modules: box area = function count; small modules: "
        "fixed cards with every function drawn; dots = functions, address-ordered"
    )
    if min_tier:
        subtitle += f"; showing {TIER_NAMES[min_tier]}+ only"
    scene = _chrome(ds, cfg, height, subtitle)

    total_major = sum(len(m.cells) for m in majors) or 1
    areas = [max(len(m.cells), 1) * content_w * major_h / total_major for m in majors]
    rects = _squarify(areas, cfg.margin, cfg.header_h, content_w, major_h)
    for mod, (x, y, w, h) in zip(majors, rects):
        _major_box(scene, mod, x, y, w, h, cfg, min_tier)

    if minors:
        band_y = cfg.header_h + major_h
        scene.add(
            Text(
                cfg.margin, band_y + 12,
                f"small modules — fixed cards, every function drawn ({len(minors)} modules ≤ {minor_max} fns)",
                cfg.small_size, cfg.muted,
            )
        )
        for i, mod in enumerate(minors):
            x = cfg.margin + (i % per_row) * (bw + 6)
            y = band_y + 18 + (i // per_row) * (bh + 6)
            _minor_card(scene, mod, x, y, bw, bh, card_cols, card_rows, cfg, min_tier)

    _legend(scene, ds, cfg, height - cfg.legend_h + 12, min_tier)
    return scene


# --------------------------------------------------------------- module grid


def _grid_of(mod: Module, x: float, y: float, cols: int, cell: float, cfg: Config, min_tier: int) -> Cells:
    tiers, base, astride = _filter_cells(mod, min_tier)
    return Cells(
        x,
        y,
        cell,
        max(1, cols),
        tiers,
        gap=cfg.gap,
        addr_stride=astride,
        base_addr=base,
        module=mod.name,
        tip=f"{mod.name} — {len(tiers):,} cells, address-ordered (hover the page for details)",
        attrs=(("data-module", mod.name),),
    )


def grid_scene(ds: Dataset, module_name: str, cfg: Config, min_tier: int = 0, compact: bool = False) -> Scene:
    """One module's functions, address-ordered, with per-module stats header."""
    mod = ds.module(module_name)
    cells = mod.cells
    n = len(cells)
    cols = cfg.grid_cols
    stride = cfg.cell + cfg.gap
    rows = math.ceil(n / cols) if n else 0
    grid_w = cols * stride - cfg.gap
    width = max(480.0, grid_w + 2 * cfg.margin)
    grid_top = cfg.header_h + 24.0
    height = grid_top + rows * stride + cfg.legend_h + cfg.margin

    counts = mod.tier_counts()
    sub = (
        f"{mod.name} — {n:,} functions · covered {_pct(mod.covered_count(), n)} · "
        f"raw {counts['raw']:,} / decompiled {counts['decompiled']:,} / "
        f"named {counts['named']:,} / verified {counts['verified']:,}"
    )
    if mod.relink_ok is not None:
        sub += f" · relink {'byte-identical' if mod.relink_ok else 'MISMATCH'}"
    if mod.imports:
        sub += f" · imports from {len(mod.imports)} module(s)"
    if min_tier:
        sub += f" · showing {TIER_NAMES[min_tier]}+ only"
    scene = _chrome(ds, cfg, height, sub, width, title=not compact)

    if n:
        filtered = [c for c in cells if c.tier >= min_tier] if min_tier else cells
        base = filtered[0].addr if filtered else 0
        # address ruler
        scene.add(Rect(cfg.margin, grid_top - 8, grid_w, 1, cfg.line))
        for k in range(0, cols, 16):
            if k >= len(filtered):
                break
            cx = cfg.margin + k * stride + cfg.cell / 2
            scene.add(Text(cx, grid_top - 12, f"+0x{filtered[k].addr - base:X}", cfg.small_size, cfg.muted, anchor="middle"))
        grid = _grid_of(mod, cfg.margin, grid_top, cols, cfg.cell, cfg, min_tier)
        scene.add(grid)
        scene.add(Rect(
            grid.x - 2, grid.y - 2, grid.w + 4, grid.h + 4, "#00000000",
            stroke=cfg.border, stroke_width=1,
        ))

    _legend(scene, ds, cfg, height - cfg.legend_h + 12, min_tier)
    return scene


def corpus_grid_scene(ds: Dataset, cfg: Config, min_tier: int = 0, cols: int | None = None) -> Scene:
    """The whole corpus as one address-ordered grid, per-module sections.

    Every function of every module in one scannable column of labeled
    sections (no pan/zoom needed — the page just scrolls).
    """
    cols = cols or cfg.corpus_cols
    cell = 10.0
    stride = cell + cfg.gap
    mods = sorted(ds.modules, key=lambda m: (-len(m.cells), m.name))
    grid_w = cols * stride - cfg.gap
    width = max(cfg.width, grid_w + 2 * cfg.margin)

    height = cfg.header_h
    layout: list[tuple[Module, float, int]] = []  # (module, y, filtered count)
    for m in mods:
        fn = len([c for c in m.cells if c.tier >= min_tier]) if min_tier else len(m.cells)
        rows = math.ceil(fn / cols) if fn else 0
        layout.append((m, height, fn))
        height += 26 + rows * stride + 14  # header band + grid + gap
    height += cfg.legend_h + cfg.margin
    scene = _chrome(ds, cfg, height, "corpus grid — every function, grouped by module (biggest first)", width)
    if min_tier:
        scene.add(Text(cfg.margin, cfg.header_h + 12, f"filter active: showing {TIER_NAMES[min_tier]}+ only", cfg.small_size, HINT_COLOR))

    for mod, y, fn in layout:
        counts = mod.tier_counts()
        head = (
            f"{mod.name}   {len(mod.cells):,} fns · covered {_pct(mod.covered_count(), len(mod.cells))} · "
            f"decompiled {_pct(counts['decompiled'], len(mod.cells))} · named {counts['named']:,}"
        )
        if mod.relink_ok is not None:
            head += f" · relink {'ok' if mod.relink_ok else 'MISMATCH'}"
        scene.add(Rect(cfg.margin, y, width - 2 * cfg.margin, 1, cfg.line))
        scene.add(Text(cfg.margin, y + 16, head, cfg.label_size, cfg.text, weight="bold",
                       tip=_tip_for(mod, min_tier)))
        if fn:
            grid = _grid_of(mod, cfg.margin, y + 22, cols, cell, cfg, min_tier)
            scene.add(grid)
            # exact-fit frame: the cells visibly sit inside their border
            scene.add(Rect(
                grid.x - 2, grid.y - 2, grid.w + 4, grid.h + 4, "#00000000",
                stroke=cfg.border, stroke_width=1,
            ))

    _legend(scene, ds, cfg, height - cfg.legend_h + 12, min_tier)
    return scene


# ------------------------------------------------------------------ bar rows


def _bar_rows_scene(
    ds: Dataset,
    rows: list[tuple[str, dict[str, int], int, str | None]],
    cfg: Config,
    subtitle: str,
    highlight_mismatch: bool,
    details: list[str] | None = None,
) -> Scene:
    """Shared renderer for overview + namespaces: one 100%-stacked
    composition bar per row. Absolute counts live in the right-hand detail
    column and the tooltips. ``details`` optionally overrides the right-hand
    text per row."""
    height = cfg.header_h + len(rows) * cfg.row_h + cfg.legend_h + cfg.margin
    scene = _chrome(ds, cfg, height, subtitle)
    bar_x = cfg.margin + cfg.label_w
    bar_w = cfg.width - cfg.margin - bar_x - cfg.stats_w

    y = cfg.header_h + 4.0
    for ri, (label, counts, total, module_name) in enumerate(rows):
        fill = cfg.text
        if highlight_mismatch and module_name is not None:
            mod = ds.module(module_name)
            if mod.relink_ok is False:
                fill = HINT_COLOR
        scene.add(
            Text(cfg.margin, y + cfg.bar_h - 1, _fit(label, cfg.label_w - 12, cfg.label_size), cfg.label_size, fill)
        )
        denom = total or 1
        off = 0.0
        for tier in TIER_LADDER:
            seg = counts[tier] / denom * bar_w
            if seg <= 0:
                continue
            scene.add(Rect(bar_x + off, y, seg, cfg.bar_h, TIER_COLORS[tier]))
            off += seg
        if total:
            scene.add(Rect(bar_x, y, bar_w, cfg.bar_h, "#00000000", stroke=cfg.line, stroke_width=1))
        if details is not None and ri < len(details):
            detail = details[ri]
        else:
            detail = (
                f"{total:,} fns · covered {_pct_covered(ds, module_name)} · decompiled {_pct(counts['decompiled'], total)}"
                if module_name is not None
                else f"{total:,} fns · decompiled {_pct(counts['decompiled'], total)}"
            )
        tip_lines = [
            f"{label} — {total:,} functions",
            f"raw {counts['raw']:,} · decompiled {counts['decompiled']:,} · "
            f"named {counts['named']:,} · verified {counts['verified']:,}",
        ]
        if module_name is not None:
            tip_lines.append(_tip_for(ds.module(module_name)).split("\n", 1)[1])
        scene.add(
            Text(
                cfg.width - cfg.margin, y + cfg.bar_h - 1, detail, cfg.small_size, cfg.muted,
                anchor="end",
                tip="\n".join(tip_lines),
            )
        )
        if module_name is not None:
            # invisible full-row hit target so the treemap-style click works
            scene.add(
                Rect(cfg.margin, y, cfg.width - 2 * cfg.margin, cfg.row_h - 2, "#00000000",
                     tip="\n".join(tip_lines), attrs=(("data-module", module_name),))
            )
        y += cfg.row_h
    _legend(scene, ds, cfg, height - cfg.legend_h + 12)
    return scene


def _pct_covered(ds: Dataset, module_name: str) -> str:
    mod = ds.module(module_name)
    return _pct(mod.covered_count(), len(mod.cells))


def overview_scene(ds: Dataset, cfg: Config, min_tier: int = 0) -> Scene:
    mods = sorted(ds.modules, key=lambda m: (-len(m.cells), m.name))
    rows = [(m.name, m.tier_counts(), len(m.cells), m.name) for m in mods]
    return _bar_rows_scene(
        ds, rows, cfg,
        "overview — tier composition per module, sorted by function count", True,
    )


def namespaces_scene(ds: Dataset, cfg: Config, min_tier: int = 0) -> Scene:
    """Namespace breakdown that explains itself.

    A namespace = a subsystem name visible in *named* symbols (demangled C++
    paths like ``xy_system::bg::System`` from exports, or Ghidra label
    namespaces). Rows show where each subsystem lives (which modules) — the
    unnamed remainder is called out explicitly instead of drowning the view.
    """
    details = ds.namespace_details(min_tier)
    named_total = sum(
        1
        for m in ds.modules
        for c in m.cells
        if c.namespace and c.tier >= min_tier
    )
    cells_total = sum(1 for m in ds.modules for c in m.cells if c.tier >= min_tier)
    unnamed = details.get("(unnamed)", {"counts": {}, "modules": {}})
    rows: list[tuple[str, dict[str, int], int, str | None]] = []

    unnamed_total = sum(unnamed["counts"].values()) if unnamed["counts"] else 0
    rows.append((
        "(unnamed — no namespace yet)",
        unnamed["counts"] if unnamed["counts"] else {t: 0 for t in TIER_LADDER},
        unnamed_total,
        None,
    ))

    def _mod_list(mods: dict[str, int]) -> str:
        top = sorted(mods.items(), key=lambda kv: (-kv[1], kv[0]))[:3]
        s = ", ".join(name for name, _ in top)
        if len(mods) > 3:
            s += f" +{len(mods) - 3} modules"
        return s

    ordered = sorted(
        ((ns, d) for ns, d in details.items() if ns != "(unnamed)"),
        key=lambda kv: (-sum(kv[1]["counts"].values()), kv[0]),
    )
    top, rest = ordered[: cfg.ns_rows], ordered[cfg.ns_rows :]
    ns_details = [f"{unnamed_total:,} fns · {_pct(unnamed_total, cells_total)} of corpus"]
    for ns, d in top:
        counts = d["counts"]
        total = sum(counts.values())
        rows.append((ns, counts, total, None))
        ns_details.append(f"{total:,} fns · in {_mod_list(d['modules'])}")
    if rest:
        merged = {t: 0 for t in TIER_LADDER}
        mod_count = 0
        for _, d in rest:
            for k, v in d["counts"].items():
                merged[k] += v
            mod_count += len(d["modules"])
        merged_total = sum(merged.values())
        rows.append((f"(other, {len(rest)} namespaces)", merged, merged_total, None))
        ns_details.append(f"{merged_total:,} fns across {mod_count} modules")

    sub = (
        f"namespaces — subsystems visible in named symbols ({named_total:,} of {cells_total:,} functions "
        f"carry one; {_pct(named_total, cells_total)} named). Named code = identified game systems; "
        "the unnamed remainder is the identification backlog."
    )
    if min_tier:
        sub += f" — filter: {TIER_NAMES[min_tier]}+ only"
    return _bar_rows_scene(ds, rows, cfg, sub, False, details=ns_details)
