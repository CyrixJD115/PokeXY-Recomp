"""All presentation knobs in one place.

Every view reads geometry, colors and fonts from a single ``Config`` so the
interactive UI and both exporters cannot drift apart. CLI flags construct a
(mostly default) Config; nothing else hardcodes presentation values.
"""

from __future__ import annotations

import os
from dataclasses import dataclass, field


@dataclass
class Config:
    # canvas
    width: int = 1280
    treemap_height: int = 640
    # hybrid treemap: modules with at most this many (filtered) functions are
    # drawn as fixed-size cards below the squarified area, sized to fit ALL
    # of their cells (no "+N more" for cards)
    treemap_minor_max: int = 200
    treemap_minor_box_w: float = 150.0

    # module-grid cells (scene units = px at scale 1 / 96 dpi)
    cell: float = 11.0
    gap: float = 2.0
    grid_cols: int = 64
    # corpus grid + treemap cell densities
    corpus_cols: int = 88
    treemap_cell: float = 6.0

    # chrome
    margin: float = 24.0
    header_h: float = 62.0
    legend_h: float = 48.0

    # bar views
    row_h: float = 17.0
    bar_h: float = 10.0
    label_w: float = 210.0
    stats_w: float = 250.0
    ns_rows: int = 34

    # type
    font_family: str = "Noto Sans, DejaVu Sans, Liberation Sans, Arial, sans-serif"
    title_size: float = 19.0
    stats_size: float = 12.5
    label_size: float = 11.5
    small_size: float = 10.5

    # colors (dark theme; palette.py owns the tier ladder)
    bg: str = "#121419"
    panel: str = "#1a1d24"
    text: str = "#e8eaed"
    muted: str = "#98a1ac"
    line: str = "#272c35"
    border: str = "#3d4450"  # strong cell/box boundaries (treemap grid lines)

    # PNG rasterizer: explicit TTF path (regular weight). None = auto-discover
    # (XYHEATMAP_FONT env overrides the discovery list).
    font_path: str | None = field(
        default_factory=lambda: os.environ.get("XYHEATMAP_FONT") or None
    )
