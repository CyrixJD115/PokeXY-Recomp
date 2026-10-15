//! Pure geometry: Dataset -> Scene (port of `layout.py`). Layout functions
//! take a dataset and a Config and return a Scene of primitives — no I/O,
//! no rendering, fully deterministic.

use crate::config::Config;
use crate::dataset::{Dataset, Module};
use crate::error::{HeatmapError, Result};
use crate::fmt::{pct, thousands};
use crate::palette::{HINT_COLOR, TIER_COLORS, TIER_LADDER};
use crate::scene::{Anchor, Cells, Item, Line, Rect, Scene, Text};

/// Average glyph-width factor for the sans font (used only to decide
/// truncation; SVG anchoring does the real alignment).
const CHAR_W: f64 = 0.58;

pub const TIER_NAMES: [&str; 4] = ["raw", "decompiled", "named", "verified"];

pub fn fit(s: &str, max_w: f64, size: f64) -> String {
    if max_w <= 0.0 {
        return String::new();
    }
    if s.chars().count() as f64 * CHAR_W * size <= max_w {
        return s.to_string();
    }
    let max_chars = ((max_w / (CHAR_W * size)) as usize)
        .max(1)
        .saturating_sub(1);
    let mut t: String = s.chars().take(max_chars).collect();
    if t.ends_with(' ') {
        t.pop();
    }
    t + "…"
}

fn stats_line(ds: &Dataset) -> String {
    let total = ds.total_cells();
    let counts = ds.tier_counts();
    let mut parts = vec![
        format!("{} modules", ds.modules.len()),
        format!("{} functions", thousands(total)),
        format!("covered {}", pct(ds.covered_count(), total)),
        format!("decompiled {}", pct(counts[1], total)),
        format!("named {}", pct(counts[2], total)),
        format!("verified {}", pct(counts[3], total)),
    ];
    if let Some((ok, all)) = ds.relink_summary() {
        parts.push(format!("relink {ok}/{all}"));
    }
    parts.join(" · ")
}

fn legend(scene: &mut Scene, ds: &Dataset, cfg: &Config, y: f64, min_tier: usize) {
    let counts = ds.tier_counts();
    let mut x = cfg.margin;
    for (i, tier) in TIER_LADDER.iter().enumerate() {
        if i < min_tier {
            continue;
        }
        let label = format!("{} {}", cap(tier), thousands(counts[i]));
        scene.add(Item::Rect(Rect::new(x, y, 11.0, 11.0, TIER_COLORS[i])));
        scene.add(Item::Text(Text::new(
            x + 16.0,
            y + 9.5,
            label.clone(),
            cfg.small_size,
            cfg.muted,
        )));
        x += 16.0 + CHAR_W * cfg.small_size * label.chars().count() as f64 + 20.0;
    }
    // overlay outlines, only when present: relaxed armcc matches (excluded
    // from verified) and attempted-but-unmatched functions
    for (label, count, color) in [
        ("relaxed", ds.hint_count(), HINT_COLOR),
        (
            "attempted",
            ds.attempted_count(),
            crate::palette::ATTEMPTED_COLOR,
        ),
    ] {
        if count == 0 {
            continue;
        }
        let label = format!("{label} {}", thousands(count));
        scene.add(Item::Rect(
            Rect::new(x, y, 11.0, 11.0, cfg.panel).stroke(color, 1.4),
        ));
        scene.add(Item::Text(Text::new(
            x + 16.0,
            y + 9.5,
            label.clone(),
            cfg.small_size,
            cfg.muted,
        )));
        x += 16.0 + CHAR_W * cfg.small_size * label.chars().count() as f64 + 20.0;
    }
    scene.add(Item::Text(
        Text::new(
            cfg.width as f64 - cfg.margin,
            y + 9.5,
            "color = highest stage reached",
            cfg.small_size,
            cfg.muted,
        )
        .anchor(Anchor::End),
    ));
}

/// Python `TIER_LABELS` — capitalized tier name.
fn cap(tier: &str) -> String {
    let mut c = tier.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

fn chrome(
    ds: &Dataset,
    cfg: &Config,
    height: f64,
    subtitle: &str,
    width: Option<f64>,
    title: bool,
) -> Scene {
    let width = width.unwrap_or(cfg.width as f64);
    let mut scene = Scene::new(width, height, cfg.bg, cfg.font_family);
    if title {
        scene.add(Item::Text(
            Text::new(
                cfg.margin,
                cfg.margin + 14.0,
                ds.title.clone(),
                cfg.title_size,
                cfg.text,
            )
            .weight("bold"),
        ));
        // baseline sits above the divider line (header_h - 8) — was
        // margin+34, which put the divider straight through the text
        scene.add(Item::Text(Text::new(
            cfg.margin,
            cfg.header_h - 13.0,
            subtitle,
            cfg.small_size,
            cfg.muted,
        )));
    } else if !subtitle.is_empty() {
        // compact chrome keeps only the subtitle; the UI header has the stats.
        scene.add(Item::Text(Text::new(
            cfg.margin,
            cfg.margin + 20.0,
            subtitle,
            cfg.label_size,
            cfg.muted,
        )));
    }
    // Narrow scenes (module grids) would overlap the title with the stats
    // line, so narrow chrome drops it.
    if title && width >= 1150.0 {
        scene.add(Item::Text(
            Text::new(
                width - cfg.margin,
                cfg.margin + 14.0,
                stats_line(ds),
                cfg.stats_size,
                cfg.muted,
            )
            .anchor(Anchor::End),
        ));
    }
    // the divider only makes sense when there is chrome above it
    if title || !subtitle.is_empty() {
        scene.add(Item::Rect(Rect::new(
            cfg.margin,
            cfg.header_h - 8.0,
            width - 2.0 * cfg.margin,
            1.0,
            cfg.line,
        )));
    }
    scene
}

fn tip_for(mod_: &Module, min_tier: usize) -> String {
    let c = mod_.tier_counts();
    let mut lines = vec![
        format!(
            "{} — {} functions",
            mod_.name,
            thousands(mod_.cells.len() as u64)
        ),
        format!(
            "raw {} · decompiled {} · named {} · verified {}",
            thousands(c[0]),
            thousands(c[1]),
            thousands(c[2]),
            thousands(c[3])
        ),
        format!(
            "recomp-covered {}",
            pct(mod_.covered_count(), mod_.cells.len() as u64)
        ),
    ];
    if mod_.hint_count() > 0 || mod_.attempted_count() > 0 {
        lines.push(format!(
            "armcc: {} relaxed · {} attempted (no match yet)",
            thousands(mod_.hint_count()),
            thousands(mod_.attempted_count())
        ));
    }
    if !mod_.imports.is_empty() {
        let shown: Vec<&str> = mod_.imports.iter().take(4).map(|s| s.as_str()).collect();
        let mut line = format!("imports from: {}", shown.join(", "));
        if mod_.imports.len() > 4 {
            line += &format!(" +{} more", mod_.imports.len() - 4);
        }
        lines.push(line);
    }
    match mod_.relink_ok {
        Some(true) => lines.push("relink: byte-identical".to_string()),
        Some(false) => lines.push("relink: MISMATCH".to_string()),
        None => {}
    }
    if min_tier > 0 {
        lines.push(format!("(showing {}+ only)", TIER_NAMES[min_tier]));
    }
    lines.join("\n")
}

/// (tiers, hint overlays, attempted overlays, base_addr, stride) honouring
/// the tier filter. Overlay vectors are parallel to `tiers`.
fn filter_cells(mod_: &Module, min_tier: usize) -> (Vec<usize>, Vec<bool>, Vec<bool>, u64, u64) {
    let cells: Vec<&crate::dataset::Cell> = if min_tier > 0 {
        mod_.cells.iter().filter(|c| c.tier >= min_tier).collect()
    } else {
        mod_.cells.iter().collect()
    };
    let tiers: Vec<usize> = cells.iter().map(|c| c.tier).collect();
    let hints: Vec<bool> = cells.iter().map(|c| c.hint()).collect();
    let attempted: Vec<bool> = cells.iter().map(|c| c.attempted()).collect();
    let base = cells.first().map(|c| c.addr).unwrap_or(0);
    let stride = if cells.len() > 1 {
        cells
            .windows(2)
            .map(|w| w[1].addr - w[0].addr)
            .min()
            .unwrap_or(4)
    } else {
        4
    };
    (tiers, hints, attempted, base, stride.clamp(1, 64))
}

/// Overlay vector trimmed to the drawn capacity; empty (and thus omitted
/// from scene JSON) when no drawn cell carries the flag.
fn overlay_vec(v: &[bool], cap: usize) -> Vec<bool> {
    let trimmed = &v[..cap.min(v.len())];
    if trimmed.iter().any(|b| *b) {
        trimmed.to_vec()
    } else {
        Vec::new()
    }
}

// ------------------------------------------------------------------ treemap

/// Squarified treemap (Bruls et al.): descending areas in, rects out.
/// `areas` must sum to `w * h`. Deterministic: row growth only accepts a
/// candidate that does not worsen the worst aspect ratio.
pub fn squarify(
    areas: &[f64],
    mut x: f64,
    mut y: f64,
    mut w: f64,
    mut h: f64,
) -> Vec<(f64, f64, f64, f64)> {
    let mut out = Vec::new();
    let n = areas.len();
    fn worst(row: &[f64], short: f64) -> f64 {
        let s: f64 = row.iter().sum();
        let mx = row.iter().cloned().fold(f64::MIN, f64::max);
        let mn = row.iter().cloned().fold(f64::MAX, f64::min);
        ((short * short * mx) / (s * s)).max((s * s) / (short * short * mn))
    }
    let mut i = 0;
    while i < n {
        let short = w.min(h);
        let mut row = vec![areas[i]];
        let mut j = i + 1;
        while j < n {
            let cand: Vec<f64> = row
                .iter()
                .copied()
                .chain(std::iter::once(areas[j]))
                .collect();
            if worst(&cand, short) <= worst(&row, short) {
                row = cand;
                j += 1;
            } else {
                break;
            }
        }
        let s: f64 = row.iter().sum();
        if w >= h {
            let rw = s / h;
            let mut cy = y;
            for a in &row {
                let ch = a / rw;
                out.push((x, cy, rw, ch));
                cy += ch;
            }
            x += rw;
            w -= rw;
        } else {
            let rh = s / w;
            let mut cx = x;
            for a in &row {
                let cw = a / rh;
                out.push((cx, y, cw, rh));
                cx += cw;
            }
            y += rh;
            h -= rh;
        }
        i = j;
    }
    out
}

fn major_box(scene: &mut Scene, mod_: &Module, r: Rect, cfg: &Config, min_tier: usize) {
    let (x, y, w, h) = (r.x, r.y, r.w, r.h);
    // One squarified treemap box: label + as many function cells as fit,
    // rows computed against the label band so cells never cross the border.
    let (tiers, hints, attempted, base, astride) = filter_cells(mod_, min_tier);
    let cs = cfg.treemap_cell;
    let pitch = cs + 1.0;
    let label_h = if w > 60.0 && h > 40.0 { 15.0 } else { 0.0 };
    let cols = (((w - 8.0) / pitch) as usize).max(1);
    let rows = (((h - label_h - 6.0) / pitch) as usize).max(1);
    let capacity = cols * rows;
    let cap = capacity.min(tiers.len());
    let tip = tip_for(mod_, min_tier);
    scene.add(Item::Rect(Rect::new(x, y, w, h, cfg.panel)));
    scene.add(Item::Cells(Cells {
        x: x + 4.0,
        y: y + label_h + 3.0,
        size: cs,
        cols,
        tiers: tiers[..cap].to_vec(),
        hints: overlay_vec(&hints, cap),
        attempted: overlay_vec(&attempted, cap),
        gap: 1.0,
        addr_stride: astride,
        base_addr: base,
        module: mod_.name.clone(),
        names: Vec::new(),
        tip: Some(tip.clone()),
        attrs: vec![("data-module".to_string(), mod_.name.clone())],
    }));
    let more = tiers.len() as i64 - capacity as i64;
    if label_h > 0.0 {
        let mut count = String::new();
        if w > 130.0 {
            count = thousands(mod_.cells.len() as u64);
            if more > 0 {
                count += &format!(" (+{})", thousands(more as u64));
            }
        }
        let count_w = CHAR_W * cfg.small_size * count.chars().count() as f64 + 10.0;
        let name = fit(&mod_.name, w - 10.0 - count_w, cfg.label_size);
        scene.add(Item::Text(
            Text::new(x + 5.0, y + 11.0, name, cfg.label_size, cfg.text).weight("bold"),
        ));
        if !count.is_empty() {
            scene.add(Item::Text(
                Text::new(x + w - 5.0, y + 11.0, count, cfg.small_size, cfg.muted)
                    .weight("bold")
                    .anchor(Anchor::End),
            ));
        }
    }
    scene.add(Item::Rect(
        Rect::new(x, y, w, h, "#00000000")
            .stroke(cfg.border, 1.5)
            .tip(tip)
            .attr("data-module", &mod_.name),
    ));
}

/// A fixed-size card for a small module: label + ALL of its cells.
/// Card geometry is derived from the minor threshold, so every card has
/// enough slots — no truncation, no slivers.
fn minor_card(
    scene: &mut Scene,
    mod_: &Module,
    r: Rect,
    cols: usize,
    rows: usize,
    cfg: &Config,
    min_tier: usize,
) {
    let (x, y, bw, bh) = (r.x, r.y, r.w, r.h);
    let (tiers, hints, attempted, base, astride) = filter_cells(mod_, min_tier);
    let cs = cfg.treemap_cell;
    let capacity = cols * rows;
    let cap = capacity.min(tiers.len());
    let tip = tip_for(mod_, min_tier);
    scene.add(Item::Rect(Rect::new(x, y, bw, bh, cfg.panel)));
    scene.add(Item::Cells(Cells {
        x: x + 4.0,
        y: y + 17.0,
        size: cs,
        cols,
        tiers: tiers[..cap].to_vec(),
        hints: overlay_vec(&hints, cap),
        attempted: overlay_vec(&attempted, cap),
        gap: 1.0,
        addr_stride: astride,
        base_addr: base,
        module: mod_.name.clone(),
        names: Vec::new(),
        tip: Some(tip.clone()),
        attrs: vec![("data-module".to_string(), mod_.name.clone())],
    }));
    let more = tiers.len() as i64 - capacity as i64;
    let mut count = thousands(mod_.cells.len() as u64);
    if more > 0 {
        count += &format!(" (+{})", thousands(more as u64));
    }
    let count_w = CHAR_W * cfg.small_size * count.chars().count() as f64 + 10.0;
    scene.add(Item::Text(
        Text::new(
            x + 5.0,
            y + 12.0,
            fit(&mod_.name, bw - 10.0 - count_w, cfg.label_size),
            cfg.label_size,
            cfg.text,
        )
        .weight("bold"),
    ));
    scene.add(Item::Text(
        Text::new(x + bw - 5.0, y + 12.0, count, cfg.small_size, cfg.muted)
            .weight("bold")
            .anchor(Anchor::End),
    ));
    scene.add(Item::Rect(
        Rect::new(x, y, bw, bh, "#00000000")
            .stroke(cfg.border, 1.5)
            .tip(tip)
            .attr("data-module", &mod_.name),
    ));
}

pub fn treemap_scene(ds: &Dataset, cfg: &Config, min_tier: usize, minor_max: Option<i64>) -> Scene {
    // Hybrid treemap: majors share a squarified area = function count;
    // small modules become fixed cards below, sized to hold ALL cells.
    let minor_max = minor_max.map(|m| m.max(0)).unwrap_or(cfg.treemap_minor_max);
    let content_w = cfg.width as f64 - 2.0 * cfg.margin;
    let major_h = cfg.treemap_height;
    let pitch = cfg.treemap_cell + 1.0;

    let mut mods: Vec<&Module> = ds.modules.iter().collect();
    mods.sort_by(|a, b| b.cells.len().cmp(&a.cells.len()).then(a.name.cmp(&b.name)));
    let minors: Vec<&Module> = mods
        .iter()
        .filter(|m| m.cells.len() as i64 <= minor_max)
        .copied()
        .collect();
    let majors: Vec<&Module> = mods
        .iter()
        .filter(|m| m.cells.len() as i64 > minor_max)
        .copied()
        .collect();

    // minor card geometry: guaranteed capacity >= minor_max cells
    let bw = cfg.treemap_minor_box_w;
    let card_cols = (((bw - 8.0) / pitch) as usize).max(1);
    let card_rows = ((minor_max + card_cols as i64 - 1) / card_cols as i64) as usize;
    let bh = 17.0 + card_rows as f64 * pitch + 3.0;
    let per_row = ((content_w / (bw + 6.0)) as usize).max(1);
    let band_rows = if minors.is_empty() {
        0
    } else {
        minors.len().div_ceil(per_row)
    };
    let band_h = band_rows as f64 * (bh + 6.0) + if minors.is_empty() { 0.0 } else { 18.0 };

    let height = cfg.header_h + major_h + band_h + cfg.legend_h + cfg.margin;
    let mut subtitle = String::from(
        "treemap hybrid — big modules: box area = function count; small modules: \
         fixed cards with every function drawn; dots = functions, address-ordered",
    );
    if min_tier > 0 {
        subtitle += &format!("; showing {}+ only", TIER_NAMES[min_tier]);
    }
    let mut scene = chrome(ds, cfg, height, &subtitle, None, true);

    let total_major = majors.iter().map(|m| m.cells.len()).sum::<usize>().max(1) as f64;
    let areas: Vec<f64> = majors
        .iter()
        .map(|m| (m.cells.len().max(1) as f64) * content_w * major_h / total_major)
        .collect();
    let rects = squarify(&areas, cfg.margin, cfg.header_h, content_w, major_h);
    for (mod_, (x, y, w, h)) in majors.iter().zip(rects) {
        major_box(&mut scene, mod_, Rect::new(x, y, w, h, ""), cfg, min_tier);
    }

    if !minors.is_empty() {
        let band_y = cfg.header_h + major_h;
        scene.add(Item::Text(Text::new(
            cfg.margin,
            band_y + 12.0,
            format!(
                "small modules — fixed cards, every function drawn ({} modules ≤ {} fns)",
                minors.len(),
                minor_max
            ),
            cfg.small_size,
            cfg.muted,
        )));
        for (i, mod_) in minors.iter().enumerate() {
            let x = cfg.margin + (i % per_row) as f64 * (bw + 6.0);
            let y = band_y + 18.0 + (i / per_row) as f64 * (bh + 6.0);
            minor_card(
                &mut scene,
                mod_,
                Rect::new(x, y, bw, bh, ""),
                card_cols,
                card_rows,
                cfg,
                min_tier,
            );
        }
    }

    legend(&mut scene, ds, cfg, height - cfg.legend_h + 12.0, min_tier);
    scene
}

// --------------------------------------------------------------- module grid

fn grid_of(
    mod_: &Module,
    x: f64,
    y: f64,
    cols: usize,
    cell: f64,
    cfg: &Config,
    min_tier: usize,
) -> Cells {
    let (tiers, hints, attempted, base, astride) = filter_cells(mod_, min_tier);
    let count = tiers.len();
    Cells {
        x,
        y,
        size: cell,
        cols: cols.max(1),
        tiers,
        hints: overlay_vec(&hints, count),
        attempted: overlay_vec(&attempted, count),
        gap: cfg.gap,
        addr_stride: astride,
        base_addr: base,
        module: mod_.name.clone(),
        names: Vec::new(),
        tip: Some(format!(
            "{} — {} cells, address-ordered (hover the page for details)",
            mod_.name,
            thousands(count as u64)
        )),
        attrs: vec![("data-module".to_string(), mod_.name.clone())],
    }
}

pub fn grid_scene(
    ds: &Dataset,
    module_name: &str,
    cfg: &Config,
    min_tier: usize,
    compact: bool,
) -> Result<Scene> {
    // One module's functions, address-ordered, with per-module stats header.
    let mod_ = ds.module(module_name)?;
    let cells = &mod_.cells;
    let n = cells.len();
    let cols = cfg.grid_cols as usize;
    let stride = cfg.cell + cfg.gap;
    let rows = if n == 0 { 0 } else { n.div_ceil(cols) };
    let grid_w = cols as f64 * stride - cfg.gap;
    let width = 480.0f64.max(grid_w + 2.0 * cfg.margin);
    // bare module pages carry a facts card in the UI — only a tier-filter
    // note (if any) still needs in-scene chrome, and with neither the grid
    // starts right at the top margin
    let bare_untiered = compact && min_tier == 0;
    let grid_top = if bare_untiered {
        cfg.margin
    } else {
        cfg.header_h + 24.0
    };
    let height = grid_top + rows as f64 * stride + cfg.legend_h + cfg.margin;

    let counts = mod_.tier_counts();
    let mut sub = format!(
        "{} — {} functions · covered {} · raw {} / decompiled {} / named {} / verified {}",
        mod_.name,
        thousands(n as u64),
        pct(mod_.covered_count(), n as u64),
        thousands(counts[0]),
        thousands(counts[1]),
        thousands(counts[2]),
        thousands(counts[3]),
    );
    match mod_.relink_ok {
        Some(true) => sub += " · relink byte-identical",
        Some(false) => sub += " · relink MISMATCH",
        None => {}
    }
    if !mod_.imports.is_empty() {
        sub += &format!(" · imports from {} module(s)", mod_.imports.len());
    }
    if min_tier > 0 {
        sub += &format!(" · showing {}+ only", TIER_NAMES[min_tier]);
    }
    let scene_sub = if bare_untiered {
        String::new()
    } else if compact {
        format!("showing {}+ only", TIER_NAMES[min_tier])
    } else {
        sub.clone()
    };
    let mut scene = chrome(ds, cfg, height, &scene_sub, Some(width), !compact);

    if n > 0 {
        let filtered: Vec<&crate::dataset::Cell> = if min_tier > 0 {
            cells.iter().filter(|c| c.tier >= min_tier).collect()
        } else {
            cells.iter().collect()
        };
        let base = filtered.first().map(|c| c.addr).unwrap_or(0);
        // address ruler
        scene.add(Item::Rect(Rect::new(
            cfg.margin,
            grid_top - 8.0,
            grid_w,
            1.0,
            cfg.line,
        )));
        for k in (0..cols).step_by(16) {
            if k >= filtered.len() {
                break;
            }
            let cx = cfg.margin + k as f64 * stride + cfg.cell / 2.0;
            scene.add(Item::Text(
                Text::new(
                    cx,
                    grid_top - 12.0,
                    format!("+0x{:X}", filtered[k].addr - base),
                    cfg.small_size,
                    cfg.muted,
                )
                .anchor(Anchor::Middle),
            ));
        }
        let grid = grid_of(mod_, cfg.margin, grid_top, cols, cfg.cell, cfg, min_tier);
        scene.add(Item::Rect(
            Rect::new(
                grid.x - 2.0,
                grid.y - 2.0,
                grid.w() + 4.0,
                grid.h() + 4.0,
                "#00000000",
            )
            .stroke(cfg.border, 1.0),
        ));
        scene.add(Item::Cells(grid));
    }

    legend(&mut scene, ds, cfg, height - cfg.legend_h + 12.0, min_tier);
    Ok(scene)
}

pub fn corpus_grid_scene(ds: &Dataset, cfg: &Config, min_tier: usize, cols: Option<i64>) -> Scene {
    // The whole corpus as one address-ordered grid, per-module sections.
    let cols = cols.unwrap_or(cfg.corpus_cols) as usize;
    let cell = 10.0;
    let stride = cell + cfg.gap;
    let mut mods: Vec<&Module> = ds.modules.iter().collect();
    mods.sort_by(|a, b| b.cells.len().cmp(&a.cells.len()).then(a.name.cmp(&b.name)));
    let grid_w = cols as f64 * stride - cfg.gap;
    let width = (cfg.width as f64).max(grid_w + 2.0 * cfg.margin);

    let mut height = cfg.header_h;
    let mut layout: Vec<(&Module, f64, usize)> = Vec::new(); // (module, y, filtered count)
    for m in &mods {
        let fn_count = if min_tier > 0 {
            m.cells.iter().filter(|c| c.tier >= min_tier).count()
        } else {
            m.cells.len()
        };
        let rows = if fn_count == 0 {
            0
        } else {
            fn_count.div_ceil(cols)
        };
        layout.push((m, height, fn_count));
        height += 26.0 + rows as f64 * stride + 14.0; // header band + grid + gap
    }
    height += cfg.legend_h + cfg.margin;
    let mut scene = chrome(
        ds,
        cfg,
        height,
        "corpus grid — every function, grouped by module (biggest first)",
        Some(width),
        true,
    );
    if min_tier > 0 {
        scene.add(Item::Text(Text::new(
            cfg.margin,
            cfg.header_h + 12.0,
            format!("filter active: showing {}+ only", TIER_NAMES[min_tier]),
            cfg.small_size,
            HINT_COLOR,
        )));
    }

    for (mod_, y, fn_count) in layout {
        let counts = mod_.tier_counts();
        let mut head = format!(
            "{}   {} fns · covered {} · decompiled {} · named {} · verified {}",
            mod_.name,
            thousands(mod_.cells.len() as u64),
            pct(mod_.covered_count(), mod_.cells.len() as u64),
            pct(counts[1], mod_.cells.len() as u64),
            thousands(counts[2]),
            thousands(counts[3]),
        );
        match mod_.relink_ok {
            Some(true) => head += " · relink ok",
            Some(false) => head += " · relink MISMATCH",
            None => {}
        }
        scene.add(Item::Rect(Rect::new(
            cfg.margin,
            y,
            width - 2.0 * cfg.margin,
            1.0,
            cfg.line,
        )));
        scene.add(Item::Text(
            Text::new(cfg.margin, y + 16.0, head, cfg.label_size, cfg.text)
                .weight("bold")
                .tip(tip_for(mod_, min_tier)),
        ));
        if fn_count > 0 {
            let grid = grid_of(mod_, cfg.margin, y + 22.0, cols, cell, cfg, min_tier);
            // exact-fit frame: the cells visibly sit inside their border
            scene.add(Item::Rect(
                Rect::new(
                    grid.x - 2.0,
                    grid.y - 2.0,
                    grid.w() + 4.0,
                    grid.h() + 4.0,
                    "#00000000",
                )
                .stroke(cfg.border, 1.0),
            ));
            scene.add(Item::Cells(grid));
        }
    }

    legend(&mut scene, ds, cfg, height - cfg.legend_h + 12.0, min_tier);
    scene
}

// ------------------------------------------------------------------ progress

/// The whole corpus as one progress banner: a 100%-stacked tier-composition
/// bar (raw → decompiled → named → verified) with in-segment percentages,
/// plus a slim recomp-covered progress bar — the at-a-glance numbers the
/// README embeds as `heatmap-progress-bar.png`.
pub fn progress_scene(ds: &Dataset, cfg: &Config) -> Scene {
    let bar_x = cfg.margin;
    let bar_w = cfg.width as f64 - 2.0 * cfg.margin;
    let bar_y = cfg.header_h + 18.0;
    let bar_h = 46.0;

    let total = ds.total_cells();
    let counts = ds.tier_counts();
    let denom = total.max(1) as f64;

    let height = bar_y + bar_h + 56.0 + cfg.legend_h + cfg.margin;
    let subtitle = "overall progress — every function of the corpus at its highest stage reached";
    let mut scene = chrome(ds, cfg, height, subtitle, None, true);

    // main stacked bar, ladder order
    let mut off = 0.0;
    for (tier, name) in TIER_LADDER.iter().enumerate() {
        let seg_w = counts[tier] as f64 / denom * bar_w;
        if seg_w <= 0.0 {
            continue;
        }
        scene.add(Item::Rect(Rect::new(
            bar_x + off,
            bar_y,
            seg_w,
            bar_h,
            TIER_COLORS[tier],
        )));
        // in-segment label: "name NN.N%" when it fits, bare percentage when
        // only that fits; dark text on the colored rungs, light on grey
        let pct_s = crate::fmt::pct(counts[tier], total);
        let wide = format!("{} {}", name, pct_s);
        let label = if CHAR_W * cfg.label_size * wide.chars().count() as f64 + 16.0 <= seg_w {
            wide
        } else if CHAR_W * cfg.label_size * pct_s.chars().count() as f64 + 12.0 <= seg_w {
            pct_s
        } else {
            String::new()
        };
        if !label.is_empty() {
            let text_fill = if *name == "raw" { cfg.text } else { "#10141a" };
            scene.add(Item::Text(
                Text::new(
                    bar_x + off + seg_w / 2.0,
                    bar_y + bar_h / 2.0 + 4.0,
                    label,
                    cfg.label_size,
                    text_fill,
                )
                .anchor(Anchor::Middle),
            ));
        }
        off += seg_w;
    }
    // frame so the bar reads on any background
    scene.add(Item::Rect(
        Rect::new(bar_x, bar_y, bar_w, bar_h, "#00000000").stroke(cfg.line, 1.0),
    ));

    // recomp-covered progress bar (covered is a flag, not a tier — own row)
    let label_w = 150.0;
    let track_y = bar_y + bar_h + 22.0;
    let track_w = bar_w - label_w;
    let covered = ds.covered_count();
    scene.add(Item::Text(Text::new(
        cfg.margin,
        track_y + 9.0,
        "recomp-covered",
        cfg.small_size,
        cfg.muted,
    )));
    scene.add(Item::Rect(
        Rect::new(bar_x, track_y, track_w, 10.0, cfg.panel).stroke(cfg.line, 1.0),
    ));
    let fill_w = covered as f64 / denom * track_w;
    if fill_w > 0.0 {
        scene.add(Item::Rect(Rect::new(
            bar_x, track_y, fill_w, 10.0, "#2fa39e",
        )));
    }
    scene.add(Item::Text(
        Text::new(
            bar_x + track_w + 12.0,
            track_y + 9.0,
            format!("{} · {}", pct(covered, total), thousands(covered)),
            cfg.small_size,
            cfg.muted,
        )
        .anchor(Anchor::Start),
    ));

    legend(&mut scene, ds, cfg, height - cfg.legend_h + 12.0, 0);
    scene
}

// ------------------------------------------------------------------ bar rows

/// One row: (label, tier counts, total, owning module name or None).
struct BarRow {
    label: String,
    counts: [u64; 4],
    total: u64,
    module: Option<String>,
}

/// Shared renderer for overview + namespaces: one 100%-stacked composition
/// bar per row. Absolute counts live in the right-hand detail column and
/// the tooltips. `details` optionally overrides the right-hand text per row.
fn bar_rows_scene(
    ds: &Dataset,
    rows: Vec<BarRow>,
    cfg: &Config,
    subtitle: &str,
    highlight_mismatch: bool,
    details: Option<Vec<String>>,
) -> Result<Scene> {
    let height = cfg.header_h + rows.len() as f64 * cfg.row_h + cfg.legend_h + cfg.margin;
    let mut scene = chrome(ds, cfg, height, subtitle, None, true);
    let bar_x = cfg.margin + cfg.label_w;
    let bar_w = cfg.width as f64 - cfg.margin - bar_x - cfg.stats_w;

    let mut y = cfg.header_h + 4.0;
    for (ri, row) in rows.iter().enumerate() {
        let mut fill = cfg.text.to_string();
        if highlight_mismatch {
            if let Some(name) = &row.module {
                if ds.module(name)?.relink_ok == Some(false) {
                    fill = HINT_COLOR.to_string();
                }
            }
        }
        scene.add(Item::Text(Text::new(
            cfg.margin,
            y + cfg.bar_h - 1.0,
            fit(&row.label, cfg.label_w - 12.0, cfg.label_size),
            cfg.label_size,
            fill,
        )));
        let denom = row.total.max(1) as f64;
        let mut off = 0.0;
        for (tier, _) in TIER_LADDER.iter().enumerate() {
            let seg = row.counts[tier] as f64 / denom * bar_w;
            if seg <= 0.0 {
                continue;
            }
            scene.add(Item::Rect(Rect::new(
                bar_x + off,
                y,
                seg,
                cfg.bar_h,
                TIER_COLORS[tier],
            )));
            off += seg;
        }
        if row.total > 0 {
            scene.add(Item::Rect(
                Rect::new(bar_x, y, bar_w, cfg.bar_h, "#00000000").stroke(cfg.line, 1.0),
            ));
        }
        let detail = match &details {
            Some(d) if ri < d.len() => d[ri].clone(),
            _ => {
                if let Some(name) = &row.module {
                    format!(
                        "{} fns · covered {} · decompiled {}",
                        thousands(row.total),
                        pct(
                            ds.module(name)?.covered_count(),
                            ds.module(name)?.cells.len() as u64
                        ),
                        pct(row.counts[1], row.total),
                    )
                } else {
                    format!(
                        "{} fns · decompiled {}",
                        thousands(row.total),
                        pct(row.counts[1], row.total)
                    )
                }
            }
        };
        let mut tip_lines = vec![
            format!("{} — {} functions", row.label, thousands(row.total)),
            format!(
                "raw {} · decompiled {} · named {} · verified {}",
                thousands(row.counts[0]),
                thousands(row.counts[1]),
                thousands(row.counts[2]),
                thousands(row.counts[3])
            ),
        ];
        if let Some(name) = &row.module {
            let full = tip_for(ds.module(name)?, 0);
            if let Some(idx) = full.find('\n') {
                tip_lines.push(full[idx + 1..].to_string());
            }
        }
        let tip = tip_lines.join("\n");
        scene.add(Item::Text(
            Text::new(
                cfg.width as f64 - cfg.margin,
                y + cfg.bar_h - 1.0,
                detail,
                cfg.small_size,
                cfg.muted,
            )
            .anchor(Anchor::End)
            .tip(tip.clone()),
        ));
        if let Some(name) = &row.module {
            // invisible full-row hit target so the treemap-style click works
            scene.add(Item::Rect(
                Rect::new(
                    cfg.margin,
                    y,
                    cfg.width as f64 - 2.0 * cfg.margin,
                    cfg.row_h - 2.0,
                    "#00000000",
                )
                .tip(tip)
                .attr("data-module", name),
            ));
        }
        y += cfg.row_h;
    }
    legend(&mut scene, ds, cfg, height - cfg.legend_h + 12.0, 0);
    Ok(scene)
}

pub fn overview_scene(ds: &Dataset, cfg: &Config, _min_tier: usize) -> Result<Scene> {
    let mut mods: Vec<&Module> = ds.modules.iter().collect();
    mods.sort_by(|a, b| b.cells.len().cmp(&a.cells.len()).then(a.name.cmp(&b.name)));
    let rows = mods
        .into_iter()
        .map(|m| BarRow {
            label: m.name.clone(),
            counts: m.tier_counts(),
            total: m.cells.len() as u64,
            module: Some(m.name.clone()),
        })
        .collect();
    bar_rows_scene(
        ds,
        rows,
        cfg,
        "overview — tier composition per module, sorted by function count",
        true,
        None,
    )
}

pub fn namespaces_scene(ds: &Dataset, cfg: &Config, min_tier: usize) -> Result<Scene> {
    // Namespace breakdown that explains itself: a namespace = a subsystem
    // name visible in named symbols; rows show where each subsystem lives.
    let details = ds.namespace_details(min_tier);
    let named_total: u64 = ds
        .modules
        .iter()
        .flat_map(|m| m.cells.iter())
        .filter(|c| !c.namespace.is_empty() && c.tier >= min_tier)
        .count() as u64;
    let cells_total: u64 = ds
        .modules
        .iter()
        .flat_map(|m| m.cells.iter())
        .filter(|c| c.tier >= min_tier)
        .count() as u64;

    let mut rows: Vec<BarRow> = Vec::new();
    let mut ns_details: Vec<String> = Vec::new();

    let unnamed = details.iter().find(|(ns, _)| ns == "(unnamed)");
    let unnamed_counts: [u64; 4] = unnamed.map(|(_, d)| d.counts).unwrap_or([0; 4]);
    let unnamed_total: u64 = unnamed_counts.iter().sum();
    rows.push(BarRow {
        label: "(unnamed — no namespace yet)".to_string(),
        counts: unnamed_counts,
        total: unnamed_total,
        module: None,
    });
    ns_details.push(format!(
        "{} fns · {} of corpus",
        thousands(unnamed_total),
        pct(unnamed_total, cells_total)
    ));

    fn mod_list(mods: &std::collections::BTreeMap<String, u64>) -> String {
        let mut top: Vec<(&String, &u64)> = mods.iter().collect();
        top.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        let mut s = top
            .iter()
            .take(3)
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        if mods.len() > 3 {
            s += &format!(" +{} modules", mods.len() - 3);
        }
        s
    }

    let mut ordered: Vec<&(String, crate::dataset::NamespaceDetail)> =
        details.iter().filter(|(ns, _)| ns != "(unnamed)").collect();
    ordered.sort_by(|a, b| {
        let ta: u64 = a.1.counts.iter().sum();
        let tb: u64 = b.1.counts.iter().sum();
        tb.cmp(&ta).then(a.0.cmp(&b.0))
    });

    for (ns, d) in ordered.iter().take(cfg.ns_rows) {
        let total: u64 = d.counts.iter().sum();
        rows.push(BarRow {
            label: ns.clone(),
            counts: d.counts,
            total,
            module: None,
        });
        ns_details.push(format!(
            "{} fns · in {}",
            thousands(total),
            mod_list(&d.modules)
        ));
    }
    let rest = &ordered[cfg.ns_rows.min(ordered.len())..];
    if !rest.is_empty() {
        let mut merged = [0u64; 4];
        let mut mod_count = 0usize;
        for (_, d) in rest {
            for (i, v) in d.counts.iter().enumerate() {
                merged[i] += v;
            }
            mod_count += d.modules.len();
        }
        let merged_total: u64 = merged.iter().sum();
        rows.push(BarRow {
            label: format!("(other, {} namespaces)", rest.len()),
            counts: merged,
            total: merged_total,
            module: None,
        });
        ns_details.push(format!(
            "{} fns across {} modules",
            thousands(merged_total),
            mod_count
        ));
    }

    let mut sub = format!(
        "namespaces — subsystems visible in named symbols ({} of {} functions carry one; {} named). \
         Named code = identified game systems; the unnamed remainder is the identification backlog.",
        thousands(named_total),
        thousands(cells_total),
        pct(named_total, cells_total)
    );
    if min_tier > 0 {
        sub += &format!(" — filter: {}+ only", TIER_NAMES[min_tier]);
    }
    bar_rows_scene(ds, rows, cfg, &sub, false, Some(ns_details))
}

// re-export for views.rs error text
pub fn unknown_view_err(view: &str) -> HeatmapError {
    HeatmapError::new(format!("unknown view {view:?}"))
}

/// re-exported Line for parity with scene.py's import surface
pub type SceneLine = Line;

#[cfg(test)]
mod tests;
