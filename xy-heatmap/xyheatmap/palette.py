"""The status-tier color ladder shared by every view and both exporters.

A function cell's color is the *highest* rung it has reached:

    raw -> decompiled -> named -> verified

Two per-cell facts are deliberately NOT tiers (they are overlays/flags,
surfaced in tooltips and statistics, never as a color step):

* ``covered``  — the static recompiler lifted this function
* ``hint``     — the runtime interpreter had to step in (red ring)
"""

from __future__ import annotations

TIER_LADDER: tuple[str, ...] = ("raw", "decompiled", "named", "verified")

TIER_LABELS: dict[str, str] = {
    "raw": "Raw",
    "decompiled": "Decompiled",
    "named": "Named",
    "verified": "Verified",
}

TIER_COLORS: dict[str, str] = {
    "raw": "#454b57",
    "decompiled": "#2fa39e",
    "named": "#58b368",
    "verified": "#eec84a",
}

HINT_COLOR = "#e05252"

# Convenience index lookups (dataset stores the ladder index per cell).
TIER_INDEX: dict[str, int] = {name: i for i, name in enumerate(TIER_LADDER)}
TIER_BY_INDEX: dict[int, str] = dict(enumerate(TIER_LADDER))


def tier_color(index: int) -> str:
    """Color for a tier index (clamped to the ladder)."""
    return TIER_COLORS[TIER_BY_INDEX[max(0, min(index, len(TIER_LADDER) - 1))]]
