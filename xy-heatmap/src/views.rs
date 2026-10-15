//! View names -> Scenes. Shared by the CLI exporter and the web server so
//! there is exactly one definition of every view (port of `views.py`).

use crate::config::Config;
use crate::dataset::Dataset;
use crate::error::{HeatmapError, Result};
use crate::layout::{
    corpus_grid_scene, grid_scene, namespaces_scene, overview_scene, treemap_scene,
};
use crate::scene::Scene;

pub const VIEW_NAMES: [&str; 5] = ["treemap", "progress", "overview", "grid", "namespaces"];

/// `"treemap" | ... | "module:NAME"` -> (view, module).
pub fn parse_view(spec: &str) -> Result<(String, Option<String>)> {
    if let Some(module) = spec.strip_prefix("module:") {
        let module = module.trim();
        if module.is_empty() {
            return Err(HeatmapError::new(
                "module view needs a name: --view module:DllBag",
            ));
        }
        return Ok(("module".to_string(), Some(module.to_string())));
    }
    if !VIEW_NAMES.contains(&spec) {
        return Err(HeatmapError::new(format!(
            "unknown view {spec:?}; known views: {}, module:<NAME>",
            VIEW_NAMES.join(", ")
        )));
    }
    Ok((spec.to_string(), None))
}

#[allow(clippy::too_many_arguments)]
pub fn build_scene(
    ds: &Dataset,
    view: &str,
    module: Option<&str>,
    cfg: &Config,
    min_tier: usize,
    cols: Option<i64>,
    compact_module: bool,
    minor_max: Option<i64>,
) -> Result<Scene> {
    match view {
        "treemap" => Ok(treemap_scene(ds, cfg, min_tier, minor_max)),
        "progress" => Ok(crate::layout::progress_scene(ds, cfg)),
        "overview" => overview_scene(ds, cfg, min_tier),
        "grid" => Ok(corpus_grid_scene(ds, cfg, min_tier, cols)),
        "namespaces" => namespaces_scene(ds, cfg, min_tier),
        "module" => {
            let Some(module) = module else {
                return Err(HeatmapError::new("module view needs a module name"));
            };
            grid_scene(ds, module, cfg, min_tier, compact_module)
        }
        other => Err(crate::layout::unknown_view_err(other)),
    }
}

pub fn view_filename(view: &str, module: Option<&str>, ext: &str) -> String {
    let base = if view == "module" {
        format!("module-{}", module.unwrap_or("").replace('/', "_"))
    } else {
        view.to_string()
    };
    format!("{base}.{ext}")
}
