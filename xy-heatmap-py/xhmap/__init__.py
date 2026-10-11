"""xhmap — the Pokémon X/Y recomp/decomp heatmap toolkit.

Package layout:
  cli.py       argument parsing + subcommands (build/serve/summary/roms)
  romdetect.py .3ds/.cxi identification (title ID -> game/kind)
  collect.py   gathers per-function status from all Source_Extracted inputs
  page.py      emits the single-page dashboard (HTML + JSON + summary.md)
  server.py    --watch fingerprint loop + --serve http server

Entry point: ``python3 xy-heatmap-py/heatmap.py`` (or ``uv run heatmap.py``
from inside the directory). See README.md for the full command surface.
"""
import os

# Anchor defaults to the repo root so the CLI runs correctly from any
# working directory (repo root, xy-heatmap-py/, editors with another cwd).
# xhmap/__init__.py -> xhmap/ -> xy-heatmap-py/ -> repo root.
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(
    os.path.abspath(__file__))))
DEFAULT_SRC = os.path.join(ROOT, ".Data", "Source_Extracted")
DEFAULT_OUT = os.path.join(DEFAULT_SRC, "heatmap_gui")

__version__ = "2.0.0"
