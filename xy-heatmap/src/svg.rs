//! Scene -> SVG (port of `svg.py`). A hand-written serializer: no library,
//! no timestamps, fixed attribute order — identical input always yields
//! byte-identical output. Rects and cell paths are marked
//! `shape-rendering="crispEdges"`; every primitive carrying a `tip` embeds
//! a native `<title>` so even static exported files hover. `Cells` groups
//! render as one `<path>` per tier and carry `data-module`/`data-cells`
//! attributes so the web UI can compute per-cell hover readouts without
//! 70k DOM titles.

use crate::palette::tier_color;
use crate::scene::{Cells, Item, Scene};

/// Compact deterministic number formatting (2 decimal places max, no
/// trailing zeros, `-0` normalized away).
pub fn num(v: f64) -> String {
    if v.is_finite() && v == v.trunc() && v.abs() < 1e15 {
        return format!("{}", v as i64);
    }
    let mut s = format!("{v:.2}");
    while s.ends_with('0') {
        s.pop();
    }
    if s.ends_with('.') {
        s.pop();
    }
    if s == "-0" {
        s = "0".to_string();
    }
    s
}

/// XML-escape text content (`&`, `<`, `>`).
pub fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// XML-escape an attribute value (`"` too).
fn escape_attr(s: &str) -> String {
    escape(s).replace('"', "&quot;")
}

fn attrs_str(extra: &[(String, String)]) -> String {
    extra
        .iter()
        .map(|(k, v)| format!(" {}=\"{}\"", escape(k), escape_attr(v)))
        .collect()
}

fn tip_el(tip: &Option<String>, indent: &str) -> String {
    match tip {
        Some(t) => format!("{indent}<title>{}</title>\n", escape(t)),
        None => String::new(),
    }
}

fn cells_svg(item: &Cells) -> String {
    // One <path> per tier, wrapped in a <g> carrying hover metadata.
    let stride = item.size + item.gap;
    let mut by_tier: Vec<(usize, Vec<usize>)> = Vec::new();
    for (i, tier) in item.tiers.iter().enumerate() {
        match by_tier.iter_mut().find(|(t, _)| t == tier) {
            Some((_, idxs)) => idxs.push(i),
            None => by_tier.push((*tier, vec![i])),
        }
    }
    by_tier.sort_by_key(|(t, _)| *t);

    let mut attrs = String::new();
    if !item.module.is_empty() {
        attrs += &format!(" data-module=\"{}\"", escape_attr(&item.module));
    }
    attrs += &format!(
        " data-cells=\"{}:{}:{}:{}:{}\"",
        item.base_addr,
        item.addr_stride,
        item.cols,
        num(stride),
        item.tiers.len()
    );
    let tip = tip_el(&item.tip, "");

    let mut parts = String::new();
    for (tier, idxs) in &by_tier {
        let seg = cells_path(idxs, item, stride);
        parts.push_str(&format!(
            "<g{attrs}>{tip}<path d=\"{seg}\" fill=\"{}\" shape-rendering=\"crispEdges\"/></g>\n",
            tier_color(*tier),
        ));
    }
    // overlay outlines (relaxed matches / attempted functions): unfocused
    // strokes on top of the tier fills, same hover metadata
    for (flags, color) in [
        (&item.hints, crate::palette::HINT_COLOR),
        (&item.attempted, crate::palette::ATTEMPTED_COLOR),
    ] {
        let idxs: Vec<usize> = flags
            .iter()
            .enumerate()
            .filter(|(_, f)| **f)
            .map(|(i, _)| i)
            .collect();
        if idxs.is_empty() {
            continue;
        }
        let seg = cells_path(&idxs, item, stride);
        parts.push_str(&format!(
            "<g{attrs}>{tip}<path d=\"{seg}\" fill=\"none\" stroke=\"{color}\" \
             stroke-width=\"1\" shape-rendering=\"crispEdges\"/></g>\n"
        ));
    }
    parts
}

/// One square subpath per cell index (shared by tier fills and overlays).
fn cells_path(idxs: &[usize], item: &Cells, stride: f64) -> String {
    let mut seg = String::new();
    for &i in idxs {
        let cx = item.x + (i % item.cols) as f64 * stride;
        let cy = item.y + (i / item.cols) as f64 * stride;
        seg.push_str(&format!(
            "M{} {}h{}v{}h-{}z",
            num(cx),
            num(cy),
            num(item.size),
            num(item.size),
            num(item.size)
        ));
    }
    seg
}

pub fn render_svg(scene: &Scene) -> Vec<u8> {
    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\" font-family=\"{}\">\n",
        num(scene.width),
        num(scene.height),
        num(scene.width),
        num(scene.height),
        escape_attr(&scene.font_family),
    ));
    out.push_str(&format!(
        "<rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\"/>\n",
        num(scene.width),
        num(scene.height),
        scene.background
    ));
    for item in &scene.items {
        match item {
            Item::Rect(r) => {
                let mut head = format!(
                    "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\"",
                    num(r.x),
                    num(r.y),
                    num(r.w),
                    num(r.h),
                    r.fill
                );
                if r.rx != 0.0 {
                    head += &format!(" rx=\"{}\"", num(r.rx));
                }
                if r.stroke.is_some() && r.stroke_width > 0.0 {
                    head += &format!(
                        " stroke=\"{}\" stroke-width=\"{}\"",
                        r.stroke.as_deref().unwrap_or(""),
                        num(r.stroke_width)
                    );
                }
                head += " shape-rendering=\"crispEdges\"";
                head += &attrs_str(&r.attrs);
                if r.tip.is_some() {
                    out.push_str(&head);
                    out.push_str(">\n");
                    out.push_str(&tip_el(&r.tip, "  "));
                    out.push_str("</rect>\n");
                } else {
                    out.push_str(&head);
                    out.push_str("/>\n");
                }
            }
            Item::Cells(c) => out.push_str(&cells_svg(c)),
            Item::Line(l) => {
                out.push_str(&format!(
                    "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/>\n",
                    num(l.x1),
                    num(l.y1),
                    num(l.x2),
                    num(l.y2),
                    l.stroke,
                    num(l.width)
                ));
            }
            Item::Text(t) => {
                let mut head = format!(
                    "<text x=\"{}\" y=\"{}\" font-size=\"{}\" fill=\"{}\" text-anchor=\"{}\"",
                    num(t.x),
                    num(t.y),
                    num(t.size),
                    t.fill,
                    t.anchor.as_str(),
                );
                if t.weight != "normal" {
                    head += &format!(" font-weight=\"{}\"", t.weight);
                }
                if let Some(family) = &t.family {
                    head += &format!(" font-family=\"{}\"", escape_attr(family));
                }
                out.push_str(&head);
                out.push('>');
                if t.tip.is_some() {
                    out.push_str(&format!(
                        "<title>{}</title>",
                        escape(t.tip.as_deref().unwrap_or(""))
                    ));
                }
                out.push_str(&format!("{}</text>\n", escape(&t.s)));
            }
        }
    }
    out.push_str("</svg>\n");
    out.into_bytes()
}
