"""Resolution-independent drawing primitives — the single rendering truth.

Every view is built once into a :class:`Scene` (a flat, paint-ordered list of
primitives). The SVG serializer, the PNG rasterizer and the web UI all
consume the same Scene, so the interactive view and every export are
geometrically identical by construction.
"""

from __future__ import annotations

from dataclasses import dataclass, field

Attrs = tuple[tuple[str, str], ...]


@dataclass(frozen=True)
class Rect:
    x: float
    y: float
    w: float
    h: float
    fill: str
    stroke: str | None = None
    stroke_width: float = 0.0
    rx: float = 0.0
    tip: str | None = None  # native hover text (SVG <title>; ignored by PNG)
    attrs: Attrs = ()       # extra attributes (e.g. data-module for the web UI)


@dataclass(frozen=True)
class Text:
    x: float
    y: float
    s: str
    size: float = 12.0
    fill: str = "#e8eaed"
    anchor: str = "start"   # start | middle | end (SVG text-anchor semantics)
    weight: str = "normal"  # normal | bold
    family: str | None = None
    tip: str | None = None


@dataclass(frozen=True)
class Line:
    x1: float
    y1: float
    x2: float
    y2: float
    stroke: str
    width: float = 1.0


@dataclass(frozen=True)
class Cells:
    """A row-wrapped grid of small status-colored cells — one per function.

    Rendered as one SVG path per tier run (compact for tens of thousands of
    cells) and as plain rects in the PNG rasterizer. Carries the coordinate
    metadata the web UI needs to compute which cell the pointer is over:

    * ``base_addr`` — address of cell 0 (cells are assumed stride ``addr_stride``)
    * ``module``    — owning module name (data-module attribute)
    * ``names``     — optional per-cell symbol names (len == len(tiers))
    """

    x: float
    y: float
    size: float
    cols: int
    tiers: tuple[int, ...]              # ladder index per cell
    gap: float = 2.0
    addr_stride: int = 4
    base_addr: int = 0
    module: str = ""
    names: tuple[str, ...] = ()
    tip: str | None = None
    attrs: Attrs = ()

    @property
    def rows(self) -> int:
        return max(1, -(-len(self.tiers) // self.cols)) if self.tiers else 1

    @property
    def w(self) -> float:
        return self.cols * (self.size + self.gap) - self.gap

    @property
    def h(self) -> float:
        return self.rows * (self.size + self.gap) - self.gap


@dataclass
class Scene:
    width: float
    height: float
    background: str
    font_family: str = "sans-serif"
    items: list[Rect | Text | Line | Cells] = field(default_factory=list)

    def add(self, *items: Rect | Text | Line | Cells) -> "Scene":
        self.items.extend(items)
        return self
