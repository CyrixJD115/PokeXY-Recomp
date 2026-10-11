//! xy-viz — README visualization assets for the RDComp heatmap.
//!
//! Reads the collector's `heatmap_data.json` (see `xy-heatmap-py/xhmap`)
//! and renders two committed assets with [charming] (ECharts for Rust):
//!
//! - `heatmap-treemap.svg`   the central treemap: one box per CRO module
//!   (area ∝ function count), each box subdivided into tier-coloured
//!   sub-boxes (raw / decompiled / identified / verified)
//! - `heatmap-progress.svg`  stacked status-tier progress bar + legend
//!
//! ```text
//! cargo build --release -p xy-viz
//! xy-viz --data .Data/Source_Extracted/heatmap_gui/heatmap_data.json \
//!        --out docs/assets
//! ```
//!
//! `xy-heatmap-py/heatmap.py readme` runs this automatically after
//! refreshing the JSON (the same shell-out pattern the `xy media`
//! commands use for ffmpeg).

use charming::{
    component::{Axis, Grid, Legend, Title},
    element::{AxisLabel, AxisType, ItemStyle, Label},
    series::{Bar, Treemap, TreemapNode},
    Chart, ImageRenderer,
};
use clap::Parser;
use serde::Deserialize;

/// Tier ladder — keep in sync with xy-heatmap-py/xhmap/assets.py COLORS.
const TIERS: &[(&str, &str, u8)] = &[
    ("Natively Verified", "#f0b429", 4),
    ("Identified / Named", "#2ecc71", 1),
    ("Decompiled to C (unnamed)", "#2b6f8a", 2),
    ("Raw Binary", "#414650", 0),
];

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Totals {
    functions: usize,
    named: usize,
    decompiled: usize,
    verified: usize,
    recomp: usize,
    modules: usize,
    link_verified: usize,
}

#[derive(Deserialize)]
struct Module {
    name: String,
    n: usize,
    st: Vec<u8>,
}

#[derive(Deserialize)]
struct HeatmapData {
    generated: String,
    game: String,
    totals: Totals,
    modules: Vec<Module>,
}

impl Module {
    fn tier_counts(&self) -> [usize; 4] {
        // verified, named, decomp, raw — same exclusive counting as the
        // Python/GUI side (bit 4 > 1 > 2 > 0; hint overlay ignored here)
        let mut c = [0usize; 4];
        for &s in &self.st {
            if s & 4 != 0 {
                c[0] += 1;
            } else if s & 1 != 0 {
                c[1] += 1;
            } else if s & 2 != 0 {
                c[2] += 1;
            } else {
                c[3] += 1;
            }
        }
        c
    }
}

#[derive(Parser)]
struct Args {
    /// heatmap_data.json produced by `xy-heatmap-py/heatmap.py build`
    #[arg(long)]
    data: String,
    /// output directory for the two SVGs
    #[arg(long, default_value = "docs/assets")]
    out: String,
    /// treemap width in px (height: treemap 640, progress 190)
    #[arg(long, default_value_t = 1000)]
    width: u32,
    #[arg(long, default_value_t = 640)]
    height: u32,
}

fn pct(n: usize, total: usize) -> String {
    format!("{:.1}%", 100.0 * n as f64 / total.max(1) as f64)
}

fn progress_chart(d: &HeatmapData) -> Chart {
    let t = &d.totals;
    let total = t.functions.max(1);
    let raw = total - t.named - t.decompiled - t.verified;

    let subtext = format!(
        "identified {} ({}) · decompiled {} ({}) · raw {} ({}) · \
         recomp-covered {} ({}) · link-ok {}/{} · generated {}",
        t.named,
        pct(t.named, total),
        t.decompiled,
        pct(t.decompiled, total),
        raw,
        pct(raw, total),
        t.recomp,
        pct(t.recomp, total),
        t.link_verified,
        t.modules,
        d.generated,
    );

    let mut chart = Chart::new()
        .background_color("#0f1115")
        .title(
            Title::new()
                .text(format!("{} — recomp/decomp progress", d.game))
                .subtext(subtext)
                .left("left")
                .top("top")
                .text_style(
                    charming::element::TextStyle::new()
                        .color("#e8e8e8")
                        .font_size(14),
                )
                .subtext_style(
                    charming::element::TextStyle::new()
                        .color("#9aa0aa")
                        .font_size(11),
                ),
        )
        .legend(
            Legend::new()
                .bottom("bottom")
                .left("left")
                .item_width(14)
                .item_height(9)
                .text_style(
                    charming::element::TextStyle::new()
                        .color("#9aa0aa")
                        .font_size(11),
                ),
        )
        .grid(Grid::new().left(16).right(16).top(64).bottom(46))
        .x_axis(
            Axis::new()
                .type_(AxisType::Value)
                .max(total as f64)
                .show(false),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec![""])
                .show(false)
                .axis_label(AxisLabel::new().show(false)),
        );
    for (label, color, bit) in TIERS {
        let count: usize = d
            .modules
            .iter()
            .map(|m| {
                let c = m.tier_counts();
                match bit {
                    4 => c[0],
                    1 => c[1],
                    2 => c[2],
                    _ => c[3],
                }
            })
            .sum();
        if count == 0 {
            continue;
        }
        chart = chart.series(
            Bar::new()
                .name(*label)
                .stack("tiers")
                .bar_width("100%")
                .item_style(ItemStyle::new().color(*color))
                .label(Label::new().show(false))
                .data(vec![count as f64]),
        );
    }
    chart
}

fn treemap_chart(d: &HeatmapData, width: u32, height: u32) -> Chart {
    let nodes: Vec<TreemapNode> = d
        .modules
        .iter()
        .map(|m| {
            let c = m.tier_counts();
            let children: Vec<TreemapNode> = TIERS
                .iter()
                .zip(c.iter())
                .filter(|(_, &count)| count > 0)
                .map(|((label, color, _), &count)| {
                    TreemapNode::new()
                        .name(*label)
                        .value(count as f64)
                        .item_style(ItemStyle::new().color(*color))
                        .label(Label::new().show(false))
                })
                .collect();
            TreemapNode::new()
                .name(m.name.clone())
                .value(m.n as f64)
                .children(children)
                // tiny modules: children can't fit, so the parent box must
                // carry a colour (raw grey) instead of the default palette
                .item_style(ItemStyle::new().color("#414650"))
        })
        .collect();

    let t = &d.totals;
    Chart::new()
        .background_color("#101216")
        .title(
            Title::new()
                .text("Central treemap — every module a box")
                .subtext(format!(
                    "{} functions · {} modules · identified {} ({}) · {}",
                    t.functions,
                    t.modules,
                    t.named,
                    pct(t.named, t.functions.max(1)),
                    d.generated,
                ))
                .left("left")
                .top("top")
                .text_style(
                    charming::element::TextStyle::new()
                        .color("#e8e8e8")
                        .font_size(14),
                )
                .subtext_style(
                    charming::element::TextStyle::new()
                        .color("#9aa0aa")
                        .font_size(11),
                ),
        )
        .series(
            Treemap::new()
                .width(width as f64 - 24.0)
                .height(height as f64 - 84.0)
                .left(12.0)
                .top(64.0)
                .data(nodes)
                .breadcrumb(serde_json::json!({ "show": false }))
                .label(Label::new().show(false))
                .upper_label(
                    Label::new()
                        .show(true)
                        .color("#c9ced6")
                        .font_size(10),
                )
                .item_style(
                    ItemStyle::new()
                        .border_color("#101216")
                        .border_width(1),
                ),
        )
}

fn main() {
    let args = Args::parse();
    let raw = std::fs::read_to_string(&args.data).unwrap_or_else(|e| {
        eprintln!("cannot read {}: {e}", args.data);
        std::process::exit(1);
    });
    let data: HeatmapData = serde_json::from_str(&raw).unwrap_or_else(|e| {
        eprintln!("cannot parse {}: {e}", args.data);
        std::process::exit(1);
    });
    std::fs::create_dir_all(&args.out).unwrap_or_else(|e| {
        eprintln!("cannot create {}: {e}", args.out);
        std::process::exit(1);
    });

    let mut renderer = ImageRenderer::new(args.width, 190);
    let progress = progress_chart(&data);
    let progress_path = format!("{}/heatmap-progress.svg", args.out);
    renderer.save(&progress, &progress_path).unwrap_or_else(|e| {
        eprintln!("render {progress_path}: {e}");
        std::process::exit(1);
    });

    let mut renderer = ImageRenderer::new(args.width, args.height);
    let treemap = treemap_chart(&data, args.width, args.height);
    let treemap_path = format!("{}/heatmap-treemap.svg", args.out);
    renderer.save(&treemap, &treemap_path).unwrap_or_else(|e| {
        eprintln!("render {treemap_path}: {e}");
        std::process::exit(1);
    });

    println!("wrote {progress_path} + {treemap_path}");
}
