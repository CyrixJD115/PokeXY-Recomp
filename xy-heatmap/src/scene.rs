//! Resolution-independent drawing primitives — the single rendering truth
//! (port of `scene.py`). Every view is built once into a [`Scene`] (a flat,
//! paint-ordered list of primitives). The SVG serializer, the PNG
//! rasterizer and the web UI all consume the same Scene, so the interactive
//! view and every export are geometrically identical by construction.

use serde::{Deserialize, Serialize};

pub type Attrs = Vec<(String, String)>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub fill: String,
    #[serde(default)]
    pub stroke: Option<String>,
    #[serde(default)]
    pub stroke_width: f64,
    #[serde(default)]
    pub rx: f64,
    /// native hover text (SVG `<title>`; ignored by PNG)
    #[serde(default)]
    pub tip: Option<String>,
    /// extra attributes (e.g. data-module for the web UI)
    #[serde(default)]
    pub attrs: Attrs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Anchor {
    #[default]
    Start,
    Middle,
    End,
}

impl Anchor {
    pub fn as_str(&self) -> &'static str {
        match self {
            Anchor::Start => "start",
            Anchor::Middle => "middle",
            Anchor::End => "end",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    pub x: f64,
    pub y: f64,
    pub s: String,
    #[serde(default = "default_size")]
    pub size: f64,
    #[serde(default = "default_fill")]
    pub fill: String,
    #[serde(default)]
    pub anchor: Anchor,
    /// "normal" | "bold"
    #[serde(default)]
    pub weight: String,
    #[serde(default)]
    pub family: Option<String>,
    #[serde(default)]
    pub tip: Option<String>,
}

fn default_size() -> f64 {
    12.0
}
fn default_fill() -> String {
    "#e8eaed".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub stroke: String,
    #[serde(default = "default_width")]
    pub width: f64,
}

fn default_width() -> f64 {
    1.0
}

/// A row-wrapped grid of small status-colored cells — one per function.
/// Rendered as one SVG path per tier run (compact for tens of thousands of
/// cells) and as plain rects in the PNG rasterizer. Carries the coordinate
/// metadata the web UI needs to compute which cell the pointer is over.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cells {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub cols: usize,
    /// ladder index per cell
    pub tiers: Vec<usize>,
    /// per-cell overlay flags (parallel to `tiers`): relaxed armcc matches
    /// and attempted-but-unmatched functions. Rendered as outlines; empty
    /// when the module has none (the common case).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hints: Vec<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attempted: Vec<bool>,
    #[serde(default = "default_gap")]
    pub gap: f64,
    #[serde(default = "default_stride")]
    pub addr_stride: u64,
    #[serde(default)]
    pub base_addr: u64,
    #[serde(default)]
    pub module: String,
    #[serde(default)]
    pub names: Vec<String>,
    #[serde(default)]
    pub tip: Option<String>,
    #[serde(default)]
    pub attrs: Attrs,
}

fn default_gap() -> f64 {
    2.0
}
fn default_stride() -> u64 {
    4
}

impl Cells {
    pub fn rows(&self) -> usize {
        if self.tiers.is_empty() {
            1
        } else {
            self.tiers.len().div_ceil(self.cols).max(1)
        }
    }

    pub fn w(&self) -> f64 {
        self.cols as f64 * (self.size + self.gap) - self.gap
    }

    pub fn h(&self) -> f64 {
        self.rows() as f64 * (self.size + self.gap) - self.gap
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Item {
    Rect(Rect),
    Text(Text),
    Line(Line),
    Cells(Cells),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub width: f64,
    pub height: f64,
    pub background: String,
    #[serde(default = "default_font")]
    pub font_family: String,
    #[serde(default)]
    pub items: Vec<Item>,
}

fn default_font() -> String {
    "sans-serif".to_string()
}

impl Scene {
    pub fn new(width: f64, height: f64, background: &str, font_family: &str) -> Self {
        Self {
            width,
            height,
            background: background.to_string(),
            font_family: font_family.to_string(),
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, item: Item) -> &mut Self {
        self.items.push(item);
        self
    }
}

/// Convenience constructors matching the Python dataclass defaults.
impl Rect {
    pub fn new(x: f64, y: f64, w: f64, h: f64, fill: impl Into<String>) -> Self {
        Self {
            x,
            y,
            w,
            h,
            fill: fill.into(),
            stroke: None,
            stroke_width: 0.0,
            rx: 0.0,
            tip: None,
            attrs: Attrs::new(),
        }
    }
    pub fn stroke(mut self, color: &str, width: f64) -> Self {
        self.stroke = Some(color.to_string());
        self.stroke_width = width;
        self
    }
    pub fn tip(mut self, tip: impl Into<String>) -> Self {
        self.tip = Some(tip.into());
        self
    }
    pub fn attr(mut self, k: &str, v: &str) -> Self {
        self.attrs.push((k.to_string(), v.to_string()));
        self
    }
}

impl Text {
    pub fn new(x: f64, y: f64, s: impl Into<String>, size: f64, fill: impl Into<String>) -> Self {
        Self {
            x,
            y,
            s: s.into(),
            size,
            fill: fill.into(),
            anchor: Anchor::Start,
            weight: "normal".to_string(),
            family: None,
            tip: None,
        }
    }
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }
    pub fn weight(mut self, weight: &str) -> Self {
        self.weight = weight.to_string();
        self
    }
    pub fn tip(mut self, tip: impl Into<String>) -> Self {
        self.tip = Some(tip.into());
        self
    }
}

impl Line {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64, stroke: impl Into<String>, width: f64) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            stroke: stroke.into(),
            width,
        }
    }
}
