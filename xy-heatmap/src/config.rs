//! All presentation knobs in one place (port of `config.py`). Every view
//! reads geometry, colors and fonts from a single `Config` so the
//! interactive UI and both exporters cannot drift apart.

/// Canvas + layout constants (scene units = px at scale 1 / 96 dpi).
#[derive(Debug, Clone)]
pub struct Config {
    pub width: i64,
    pub treemap_height: f64,
    /// hybrid treemap: modules with at most this many (filtered) functions
    /// are drawn as fixed-size cards below the squarified area, sized to
    /// fit ALL of their cells (no "+N more" for cards)
    pub treemap_minor_max: i64,
    pub treemap_minor_box_w: f64,

    pub cell: f64,
    pub gap: f64,
    pub grid_cols: i64,
    pub corpus_cols: i64,
    pub treemap_cell: f64,

    pub margin: f64,
    pub header_h: f64,
    pub legend_h: f64,

    // bar views
    pub row_h: f64,
    pub bar_h: f64,
    pub label_w: f64,
    pub stats_w: f64,
    pub ns_rows: usize,

    // type
    pub font_family: &'static str,
    pub title_size: f64,
    pub stats_size: f64,
    pub label_size: f64,
    pub small_size: f64,

    // colors (dark theme; palette.rs owns the tier ladder)
    pub bg: &'static str,
    pub panel: &'static str,
    pub text: &'static str,
    pub muted: &'static str,
    pub line: &'static str,
    pub border: &'static str,

    /// PNG rasterizer: explicit TTF path override (XYHEATMAP_FONT env),
    /// None = let resvg/fontdb discover system fonts.
    pub font_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 1280,
            treemap_height: 640.0,
            treemap_minor_max: 200,
            treemap_minor_box_w: 150.0,
            cell: 11.0,
            gap: 2.0,
            grid_cols: 64,
            corpus_cols: 88,
            treemap_cell: 6.0,
            margin: 24.0,
            header_h: 68.0,
            legend_h: 48.0,
            row_h: 17.0,
            bar_h: 10.0,
            label_w: 210.0,
            stats_w: 250.0,
            ns_rows: 34,
            font_family: "Noto Sans, DejaVu Sans, Liberation Sans, Arial, sans-serif",
            title_size: 19.0,
            stats_size: 12.5,
            label_size: 11.5,
            small_size: 10.5,
            bg: "#121419",
            panel: "#1a1d24",
            text: "#e8eaed",
            muted: "#98a1ac",
            line: "#272c35",
            border: "#3d4450",
            font_path: std::env::var("XYHEATMAP_FONT")
                .ok()
                .filter(|s| !s.is_empty()),
        }
    }
}
