"""Scene -> PNG.

Rasterizes the exact same primitives the SVG writer emits, at an integral
``scale`` (pixels per scene unit, scene units being px at 96 dpi):

* rects land on exact pixel boundaries (crisp cells, no AA seams)
* text renders from a real TrueType font (auto-discovered; XYHEATMAP_FONT
  overrides) with SVG-compatible anchoring
* the PNG carries pHYs DPI metadata so it has a correct physical size

Colors are #RGB / #RRGGBB; the 8-digit form #RRGGBBAA with alpha 0 means
"hit-target only, do not paint" and is skipped.
"""

from __future__ import annotations

import io
import os
from pathlib import Path

from xyheatmap.errors import HeatmapError
from xyheatmap.palette import tier_color
from xyheatmap.scene import Cells, Line, Rect, Scene, Text

_FONT_DIRS = ("/usr/share/fonts", "/usr/local/share/fonts", str(Path.home() / ".fonts"))
_FONT_CANDIDATES = (
    "noto/NotoSans-Regular.ttf",
    "dejavu/DejaVuSans.ttf",
    "liberation/LiberationSans-Regular.ttf",
    "TTF/DejaVuSans.ttf",
    "droid/DroidSans.ttf",
    "adwaita/AdwaitaSans-Regular.ttf",
)
_BOLD_REWRITE = (
    ("NotoSans-Regular", "NotoSans-Bold"),
    ("DejaVuSans", "DejaVuSans-Bold"),
    ("LiberationSans-Regular", "LiberationSans-Bold"),
    ("DroidSans", "DroidSans-Bold"),
    ("AdwaitaSans-Regular", "AdwaitaSans-Bold"),
)

_font_cache: dict[tuple[str, int], object] = {}
_font_pair: tuple[object, object, bool] | None = None  # (regular, bold, resolved)


def _first_font(names) -> str | None:
    for d in _FONT_DIRS:
        for name in names:
            p = Path(d) / name
            if p.is_file():
                return str(p)
    return None


def discover_fonts() -> tuple[str | None, str | None]:
    """(regular_ttf, bold_ttf) — either may be None if the system has none."""
    regular = _first_font(_FONT_CANDIDATES)
    if regular is None:
        return None, None
    bold_name = None
    for old, new in _BOLD_REWRITE:
        if old in regular:
            bold_name = regular.replace(old, new)
            break
    bold = bold_name if bold_name and Path(bold_name).is_file() else None
    return regular, bold


def _fonts():
    global _font_pair
    if _font_pair is None:
        from PIL import ImageFont

        env = os.environ.get("XYHEATMAP_FONT")
        if env and Path(env).is_file():
            regular = env
            bold_name = None
            for old, new in _BOLD_REWRITE:
                if old in regular:
                    bold_name = regular.replace(old, new)
                    break
            bold = bold_name if bold_name and Path(bold_name).is_file() else regular
        else:
            regular, bold = discover_fonts()
        if regular is None:
            _font_pair = (None, None, False)
        else:
            _font_pair = (regular, bold, True)
    return _font_pair


def _font(size: float, weight: str, scale: float):
    from PIL import ImageFont

    regular, bold, _ = _fonts()
    path = (bold if (weight == "bold" and bold) else regular)
    if path is None:
        key, size_px = ("default", max(6, int(round(size * scale))))
        if key not in _font_cache:
            _font_cache[key] = ImageFont.load_default(size=size_px)
        return _font_cache[key], False
    size_px = max(6, int(round(size * scale)))
    key = (path, size_px)
    if key not in _font_cache:
        _font_cache[key] = ImageFont.truetype(path, size_px)
    return _font_cache[key], True


def _rgb(hex_color: str):
    s = hex_color.lstrip("#")
    if len(s) == 8:  # RRGGBBAA
        if s[6:8] == "00":
            return None  # fully transparent hit-target
        s = s[:6]
    if len(s) == 3:
        s = "".join(ch * 2 for ch in s)
    if len(s) != 6:
        raise HeatmapError(f"unsupported color {hex_color!r}")
    return int(s[0:2], 16), int(s[2:4], 16), int(s[4:6], 16)


def render_png(scene: Scene, scale: int = 2, dpi: int | None = None, warnings: list[str] | None = None):
    """Render a Scene to a PIL Image at ``scale`` px per scene unit."""
    try:
        from PIL import Image, ImageDraw
    except ImportError:
        raise HeatmapError(
            "PNG export needs Pillow (pip install pillow) — SVG export works without it"
        ) from None

    regular, bold, _ = _fonts()
    if regular is None and warnings is not None:
        warnings.append("no TrueType font found — PNG text will use the bitmap fallback font")

    def px(v: float) -> int:
        return int(round(v * scale))

    img = Image.new("RGB", (max(1, px(scene.width)), max(1, px(scene.height))), _rgb(scene.background))
    draw = ImageDraw.Draw(img)

    for item in scene.items:
        if isinstance(item, Rect):
            fill = _rgb(item.fill)
            stroke = _rgb(item.stroke) if item.stroke and item.stroke_width > 0 else None
            if fill is None and stroke is None:
                continue
            box = (px(item.x), px(item.y), px(item.x + item.w) - 1, px(item.y + item.h) - 1)
            if box[2] < box[0] or box[3] < box[1]:
                continue
            if fill is not None and stroke is None and item.rx > 0:
                draw.rounded_rectangle(box, radius=max(1, px(item.rx)), fill=fill)
            elif fill is not None:
                draw.rectangle(box, fill=fill, outline=stroke, width=max(1, px(item.stroke_width)) if stroke else 0)
            elif stroke is not None:
                draw.rectangle(box, outline=stroke, width=max(1, px(item.stroke_width)))
        elif isinstance(item, Cells):
            stride = item.size + item.gap
            cell = max(1, px(item.size))
            gap = max(1, px(stride))
            for i, tier in enumerate(item.tiers):
                cx = px(item.x + (i % item.cols) * stride)
                cy = px(item.y + (i // item.cols) * stride)
                draw.rectangle(
                    (cx, cy, cx + cell - 1, cy + cell - 1),
                    fill=_rgb(tier_color(tier)),
                )
        elif isinstance(item, Line):
            draw.line(
                [(px(item.x1), px(item.y1)), (px(item.x2), px(item.y2))],
                fill=_rgb(item.stroke),
                width=max(1, px(item.width)),
            )
        elif isinstance(item, Text):
            fill = _rgb(item.fill)
            if fill is None:
                continue
            font, is_ttf = _font(item.size, item.weight, scale)
            anchor = ("ls" if item.anchor == "start" else "ms" if item.anchor == "middle" else "rs")
            draw.text((px(item.x), px(item.y)), item.s, fill=fill, font=font, anchor=anchor if is_ttf else None)
    img.info["dpi"] = (dpi or 96 * scale, dpi or 96 * scale)
    return img


def png_bytes(scene: Scene, scale: int = 2, dpi: int | None = None, warnings: list[str] | None = None) -> bytes:
    img = render_png(scene, scale=scale, dpi=dpi, warnings=warnings)
    buf = io.BytesIO()
    img.save(buf, format="PNG", dpi=img.info["dpi"])
    return buf.getvalue()


def png_info(scene: Scene, scale: int, dpi: int | None) -> tuple[int, int, int]:
    """(pixel width, pixel height, effective dpi) for reporting."""
    return (
        max(1, int(round(scene.width * scale))),
        max(1, int(round(scene.height * scale))),
        dpi or 96 * scale,
    )
