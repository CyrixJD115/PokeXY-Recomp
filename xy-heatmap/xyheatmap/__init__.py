"""xy-heatmap — standalone progress-heatmap studio.

Turns a module×function status dataset (collected from the recomp/decomp
corpus, or supplied as generic JSON) into:

* an interactive local web UI served entirely from memory (no generated
  HTML files anywhere on disk), and
* deterministic, high-quality SVG and PNG exports rendered from the exact
  same scene the UI displays.

Package layout:

    dataset.py   the data model (modules of address-ordered cells with
                 status tiers) + validation + generic JSON input
    collect.py   read-only collectors for the project's data sources
    config.py    one place for colors, geometry, fonts (shared by UI/export)
    palette.py   the status-tier color ladder
    scene.py     resolution-independent drawing primitives
    layout.py    pure geometry: squarified treemap, module grids, bars
    svg.py       Scene -> SVG bytes (hand-written, deterministic)
    png.py       Scene -> PNG raster (Pillow, real TTF fonts, DPI metadata)
    server.py    stdlib HTTP server hosting UI + API + rendered views
    webui.py     the single-page UI as an in-memory string
    cli.py       serve | export | summary | readme | selftest
"""

__version__ = "1.0.0"

from xyheatmap.dataset import Cell, Dataset, Module, dataset_from_dict, load_dataset
from xyheatmap.errors import HeatmapError

__all__ = [
    "Cell",
    "Dataset",
    "Module",
    "HeatmapError",
    "dataset_from_dict",
    "load_dataset",
    "__version__",
]
