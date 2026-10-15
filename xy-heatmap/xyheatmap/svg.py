"""Scene -> SVG.

A hand-written serializer: no library, no timestamps, fixed attribute order —
identical input always yields byte-identical output. Axis-aligned rects and
cell paths are marked ``shape-rendering="crispEdges"`` so boundaries stay
sharp; every primitive carrying a ``tip`` embeds a native ``<title>`` so
even static exported files hover. :class:`Cells` groups render as one
``<path>`` per tier and carry ``data-module``/``data-cells`` attributes so
the web UI can compute per-cell hover readouts without 70k DOM titles.
"""

from __future__ import annotations

from xml.sax.saxutils import escape

from xyheatmap.palette import tier_color
from xyheatmap.scene import Cells, Line, Rect, Scene, Text


def _num(v: float) -> str:
    """Compact deterministic number formatting (2 decimal places max)."""
    if isinstance(v, int):
        return str(v)
    r = round(v, 2)
    if r == int(r):
        return str(int(r))
    return repr(r)


def _attrs(extra) -> str:
    quoted = {chr(34): "&quot;"}
    return "".join(f' {k}="{escape(v, quoted)}"' for k, v in extra)


def _tip_el(tip: str | None, indent: str = "  ") -> str:
    if not tip:
        return ""
    return f"{indent}<title>{escape(tip)}</title>\n"


def _cells_svg(item: Cells) -> str:
    """One <path> per tier, wrapped in a <g> carrying hover metadata."""
    stride = item.size + item.gap
    by_tier: dict[int, list[int]] = {}
    for i, tier in enumerate(item.tiers):
        by_tier.setdefault(tier, []).append(i)

    parts: list[str] = []
    for tier in sorted(by_tier):
        seg: list[str] = []
        for i in by_tier[tier]:
            cx = item.x + (i % item.cols) * stride
            cy = item.y + (i // item.cols) * stride
            seg.append(
                f"M{_num(cx)} {_num(cy)}h{_num(item.size)}v{_num(item.size)}h-{_num(item.size)}z"
            )
        attrs = f' data-module="{escape(item.module)}"' if item.module else ""
        attrs += (
            f' data-cells="{item.base_addr}:{item.addr_stride}:{item.cols}:{_num(stride)}:{len(item.tiers)}"'
        )
        tip = _tip_el(item.tip, "")
        parts.append(
            f"<g{attrs}>{tip}"
            f'<path d="{"".join(seg)}" fill="{tier_color(tier)}" '
            'shape-rendering="crispEdges"/></g>\n'
        )
    return "".join(parts)


def render_svg(scene: Scene) -> bytes:
    out: list[str] = []
    out.append('<?xml version="1.0" encoding="UTF-8"?>\n')
    out.append(
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{_num(scene.width)}" '
        f'height="{_num(scene.height)}" viewBox="0 0 {_num(scene.width)} '
        f'{_num(scene.height)}" font-family="{escape(scene.font_family)}">\n'
    )
    out.append(
        f'<rect x="0" y="0" width="{_num(scene.width)}" '
        f'height="{_num(scene.height)}" fill="{scene.background}"/>\n'
    )
    for item in scene.items:
        if isinstance(item, Rect):
            head = (
                f'<rect x="{_num(item.x)}" y="{_num(item.y)}" width="{_num(item.w)}" '
                f'height="{_num(item.h)}" fill="{item.fill}"'
            )
            if item.rx:
                head += f' rx="{_num(item.rx)}"'
            if item.stroke and item.stroke_width > 0:
                head += f' stroke="{item.stroke}" stroke-width="{_num(item.stroke_width)}"'
            head += ' shape-rendering="crispEdges"'
            head += _attrs(item.attrs)
            if item.tip:
                out.append(head + ">\n")
                out.append(_tip_el(item.tip))
                out.append("</rect>\n")
            else:
                out.append(head + "/>\n")
        elif isinstance(item, Cells):
            out.append(_cells_svg(item))
        elif isinstance(item, Line):
            out.append(
                f'<line x1="{_num(item.x1)}" y1="{_num(item.y1)}" x2="{_num(item.x2)}" '
                f'y2="{_num(item.y2)}" stroke="{item.stroke}" '
                f'stroke-width="{_num(item.width)}"/>\n'
            )
        elif isinstance(item, Text):
            head = (
                f'<text x="{_num(item.x)}" y="{_num(item.y)}" '
                f'font-size="{_num(item.size)}" fill="{item.fill}" '
                f'text-anchor="{item.anchor}"'
            )
            if item.weight != "normal":
                head += f' font-weight="{item.weight}"'
            if item.family:
                head += f' font-family="{escape(item.family)}"'
            out.append(head + ">")
            if item.tip:
                out.append(f"<title>{escape(item.tip)}</title>")
            out.append(f"{escape(item.s)}</text>\n")
    out.append("</svg>\n")
    return "".join(out).encode("utf-8")
