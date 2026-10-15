//! Layout geometry + SVG serializer tests (port of test_layout.py /
//! test_svg.py substance): squarify invariants, the hybrid-treemap
//! minor-card invariant, determinism, compact number formatting.

use super::*;
use crate::config::Config;
use crate::dataset::{Cell, Dataset, Module};
use crate::palette::TIER_COLORS;
use crate::svg::render_svg;
use crate::views::{build_scene, parse_view, view_filename, VIEW_NAMES};

fn cell(addr: u64, tier: usize) -> Cell {
    Cell {
        addr,
        size: 4,
        tier,
        ..Default::default()
    }
}

/// 3 modules covering the tier ladder, one small enough to be a minor card.
fn sample_dataset() -> Dataset {
    let mut big = Module {
        name: "Big".into(),
        ..Default::default()
    };
    for i in 0..50 {
        big.cells.push(cell(0x180 + (i * 4) as u64, i % 4));
    }
    let mut mid = Module {
        name: "Mid".into(),
        ..Default::default()
    };
    for i in 0..20 {
        mid.cells.push(cell(0x180 + (i * 8) as u64, i % 3));
    }
    let small = Module {
        name: "Small".into(),
        cells: vec![cell(0x180, 0), cell(0x184, 2), cell(0x190, 3)],
        relink_ok: Some(false),
        imports: vec!["Big".into()],
        ..Default::default()
    };
    Dataset {
        title: "test".into(),
        modules: vec![big, mid, small],
        ..Default::default()
    }
}

#[test]
fn squarify_conserves_area_and_stays_in_bounds() {
    // areas must sum to w*h (the caller normalizes before calling)
    let raw = [6.0, 6.0, 4.0, 3.0, 2.0, 2.0, 1.0];
    let total_raw: f64 = raw.iter().sum();
    let (x, y, w, h) = (0.0, 0.0, 100.0, 50.0);
    let areas: Vec<f64> = raw.iter().map(|a| a * (w * h) / total_raw).collect();
    let rects = squarify(&areas, x, y, w, h);
    assert_eq!(rects.len(), areas.len());
    let total: f64 = rects.iter().map(|r| r.2 * r.3).sum();
    assert!((total - w * h).abs() < 1e-9, "area not conserved: {total}");
    for (rx, ry, rw, rh) in &rects {
        assert!(*rx >= x - 1e-9 && *ry >= y - 1e-9);
        assert!(*rx + *rw <= x + w + 1e-9 && *ry + *rh <= y + h + 1e-9);
    }
    // deterministic
    assert_eq!(squarify(&areas, x, y, w, h), squarify(&areas, x, y, w, h));
}

#[test]
fn squarify_single_area_fills_everything() {
    let rects = squarify(&[42.0], 0.0, 0.0, 7.0, 6.0);
    assert_eq!(rects, vec![(0.0, 0.0, 7.0, 6.0)]);
}

#[test]
fn parse_view_and_filename() {
    assert_eq!(parse_view("treemap").unwrap(), ("treemap".into(), None));
    assert_eq!(
        parse_view("module:DllBag").unwrap(),
        ("module".into(), Some("DllBag".into()))
    );
    assert!(parse_view("module:").is_err());
    assert!(parse_view("bogus").is_err());
    assert_eq!(
        view_filename("module", Some("a/b"), "svg"),
        "module-a_b.svg"
    );
    assert_eq!(view_filename("treemap", None, "png"), "treemap.png");
    assert_eq!(VIEW_NAMES.len(), 5);
}

#[test]
fn treemap_minor_cards_hold_every_cell() {
    let ds = sample_dataset();
    let cfg = Config::default();
    let scene = build_scene(&ds, "treemap", None, &cfg, 0, None, false, None).unwrap();

    // every module appears as a data-module attr (majors + minor cards)
    let mut modules_seen = std::collections::BTreeSet::new();
    let mut cell_counts: std::collections::BTreeMap<String, usize> = Default::default();
    for item in &scene.items {
        if let crate::scene::Item::Cells(c) = item {
            modules_seen.insert(c.module.clone());
            *cell_counts.entry(c.module.clone()).or_default() += c.tiers.len();
        }
    }
    assert_eq!(modules_seen.len(), 3);
    // ALL cells drawn somewhere (the minor-card invariant: no truncation
    // below the threshold, majors clip only what doesn't fit their box)
    assert_eq!(cell_counts["Small"], 3);
    assert_eq!(cell_counts["Mid"], 20);
}

#[test]
fn tier_filter_applies_to_grids() {
    let ds = sample_dataset();
    let cfg = Config::default();
    for view in ["grid", "overview", "namespaces", "treemap"] {
        let scene = build_scene(&ds, view, None, &cfg, 2, None, false, None).unwrap();
        for item in &scene.items {
            if let crate::scene::Item::Cells(c) = item {
                assert!(
                    c.tiers.iter().all(|t| *t >= 2),
                    "{view} leaked a filtered cell"
                );
            }
        }
    }
    // module view errors on an unknown module and renders a known one
    assert!(build_scene(&ds, "module", Some("Nope"), &cfg, 0, None, false, None).is_err());
    let scene = build_scene(&ds, "module", Some("Small"), &cfg, 0, None, true, None).unwrap();
    let cells: Vec<_> = scene
        .items
        .iter()
        .filter_map(|i| match i {
            crate::scene::Item::Cells(c) => Some(c),
            _ => None,
        })
        .collect();
    assert_eq!(cells.len(), 1);
    assert_eq!(cells[0].tiers.len(), 3);
}

#[test]
fn svg_is_byte_deterministic_and_compact() {
    let ds = sample_dataset();
    let cfg = Config::default();
    let scene = build_scene(&ds, "treemap", None, &cfg, 0, None, false, None).unwrap();
    let a = render_svg(&scene);
    let b = render_svg(&scene);
    assert_eq!(a, b, "SVG bytes must be deterministic");
    let s = String::from_utf8(a).unwrap();
    assert!(s.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(s.trim_end().ends_with("</svg>"));
    // number formatting: no ".0"-style floats in the body (the XML
    // declaration's version="1.0" obviously excepted)
    let body = &s[s.find('\n').unwrap() + 1..];
    assert!(!body.contains(".0 "), "unformatted float leaked");
    assert!(!body.contains(".0\""), "unformatted float leaked: …{}", {
        let i = body.find(".0\"").unwrap_or(0);
        &body[i.saturating_sub(60)..i + 3]
    });
    assert!(s.contains("data-cells=\""), "hover metadata missing");
    assert!(s.contains("data-module=\"Small\""));
    assert!(s.contains("<title>"));
    // the tier path fill uses the ladder color
    assert!(s.contains(TIER_COLORS[3]));
}

#[test]
fn number_formatting_is_compact() {
    assert_eq!(crate::svg::num(1280.0), "1280");
    assert_eq!(crate::svg::num(12.5), "12.5");
    assert_eq!(crate::svg::num(0.0), "0");
    assert_eq!(crate::svg::num(-0.0), "0");
    assert_eq!(crate::svg::num(1.005), "1"); // rounds to 1.0 at 2dp
    assert_eq!(crate::svg::num(17.039999), "17.04");
}

#[test]
fn fit_truncates_with_ellipsis() {
    assert_eq!(fit("short", 100.0, 10.0), "short");
    assert_eq!(fit("", 100.0, 10.0), "");
    assert_eq!(fit("", 0.0, 10.0), "");
    let long = "a-very-long-module-name-that-will-not-fit";
    let fitted = fit(long, 50.0, 11.5);
    assert!(fitted.ends_with('…'));
    assert!(fitted.chars().count() < long.chars().count());
}

#[test]
fn thousands_and_pct_helpers() {
    assert_eq!(crate::fmt::thousands(72867), "72,867");
    assert_eq!(crate::fmt::thousands(999), "999");
    assert_eq!(crate::fmt::pct(1, 0), "0.0%");
    assert_eq!(crate::fmt::pct(1, 3), "33.3%");
}

#[test]
fn progress_scene_bar_conserves_width_and_labels() {
    let ds = sample_dataset();
    let cfg = Config::default();
    let scene = crate::layout::progress_scene(&ds, &cfg);
    // the four tier segments tile the content width exactly
    let segs: Vec<_> = scene
        .items
        .iter()
        .filter_map(|i| match i {
            crate::scene::Item::Rect(r) if r.h == 46.0 && r.fill != "#00000000" => Some(r),
            _ => None,
        })
        .collect();
    assert_eq!(segs.len(), 4, "expected one segment per tier");
    let total_w: f64 = segs.iter().map(|r| r.w).sum();
    let content_w = cfg.width as f64 - 2.0 * cfg.margin;
    assert!(
        (total_w - content_w).abs() < 1e-9,
        "bar width not conserved"
    );
    // segment order is ladder order
    assert_eq!(segs[0].fill, TIER_COLORS[0]);
    assert_eq!(segs[3].fill, TIER_COLORS[3]);
    // covered track + labels exist; deterministic bytes via the svg renderer
    let a = crate::svg::render_svg(&scene);
    let b = crate::svg::render_svg(&scene);
    assert_eq!(a, b);
    let s = String::from_utf8(a).unwrap();
    assert!(s.contains("recomp-covered"));
    // the sample dataset has no covered cells and no verified-tier... rather:
    // covered = 0 renders an empty track with the 0.0% label
    assert!(s.contains("0.0%"));
}
