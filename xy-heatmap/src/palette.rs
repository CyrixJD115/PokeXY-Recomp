//! The status-tier color ladder shared by every view and both exporters
//! (port of `palette.py`). A cell's color is the *highest* rung it reached:
//! raw -> decompiled -> named -> verified. `covered`/`hint` are flags
//! (overlays), never color steps.

pub const TIER_LADDER: [&str; 4] = ["raw", "decompiled", "named", "verified"];

pub const TIER_COLORS: [&str; 4] = ["#454b57", "#2fa39e", "#58b368", "#eec84a"];

pub const HINT_COLOR: &str = "#e05252";

pub fn tier_index(name: &str) -> Option<usize> {
    TIER_LADDER.iter().position(|&t| t == name)
}

/// Color for a tier index (clamped to the ladder).
pub fn tier_color(index: usize) -> &'static str {
    TIER_COLORS[index.min(TIER_LADDER.len() - 1)]
}
