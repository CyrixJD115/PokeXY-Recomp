"""View names -> Scenes. Shared by the CLI exporter and the web server so
there is exactly one definition of every view."""

from __future__ import annotations

from xyheatmap.config import Config
from xyheatmap.dataset import Dataset
from xyheatmap.errors import HeatmapError
from xyheatmap.layout import (
    corpus_grid_scene,
    grid_scene,
    namespaces_scene,
    overview_scene,
    treemap_scene,
)

VIEW_NAMES = ("treemap", "overview", "grid", "namespaces")


def parse_view(spec: str) -> tuple[str, str | None]:
    """"treemap" | "overview" | "grid" | "namespaces" | "module:NAME" -> (view, module)."""
    if spec.startswith("module:"):
        module = spec[len("module:") :].strip()
        if not module:
            raise HeatmapError("module view needs a name: --view module:DllBag")
        return "module", module
    if spec not in VIEW_NAMES:
        raise HeatmapError(
            f"unknown view {spec!r}; known views: {', '.join(VIEW_NAMES)}, module:<NAME>"
        )
    return spec, None


def build_scene(
    ds: Dataset,
    view: str,
    module: str | None,
    cfg: Config,
    min_tier: int = 0,
    cols: int | None = None,
    compact_module: bool = False,
    minor_max: int | None = None,
):
    if view == "treemap":
        return treemap_scene(ds, cfg, min_tier, minor_max)
    if view == "overview":
        return overview_scene(ds, cfg, min_tier)
    if view == "grid":
        return corpus_grid_scene(ds, cfg, min_tier, cols)
    if view == "namespaces":
        return namespaces_scene(ds, cfg, min_tier)
    if view == "module":
        if not module:
            raise HeatmapError("module view needs a module name")
        return grid_scene(ds, module, cfg, min_tier, compact=compact_module)
    raise HeatmapError(f"unknown view {view!r}")


def view_filename(view: str, module: str | None, ext: str) -> str:
    base = f"module-{module.replace('/', '_')}" if view == "module" else view
    return f"{base}.{ext}"
