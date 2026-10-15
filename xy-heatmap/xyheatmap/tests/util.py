"""Shared fixtures for the xy-heatmap test suite."""

from __future__ import annotations

from xyheatmap.dataset import FLAG_COVERED, FLAG_HINT, Cell, Dataset, Module


def sample_dataset() -> Dataset:
    bag = Module(
        name="DllBag",
        relink_ok=False,
        cells=[
            Cell(addr=0x180, tier=2, flags=FLAG_COVERED, size=64),          # named + covered
            Cell(addr=0x188, tier=1, flags=FLAG_HINT, size=32),             # decompiled + hint
            Cell(addr=0x1A0, tier=3, size=16, name="DoOpen"),               # verified
        ],
    )
    battle = Module(
        name="DllBattle",
        relink_ok=True,
        cells=[Cell(addr=0x180 + i * 4, tier=(i % 4)) for i in range(40)],
    )
    title = Module(
        name="DllTitle",
        relink_ok=None,
        cells=[
            Cell(addr=0x180, tier=0, flags=FLAG_COVERED),
            Cell(addr=0x184, tier=1, namespace="ui::title"),
            Cell(addr=0x188, tier=2),
        ],
    )
    return Dataset(
        title="test corpus",
        modules=[bag, battle, title],
        warnings=["DllTitle: no export symbols"],
    )


def sample_dataset_dict() -> dict:
    return {
        "title": "test corpus",
        "modules": [
            {
                "name": "DllBag",
                "relink_ok": False,
                "cells": [
                    {"addr": "0x180", "size": 64, "covered": True, "named": True},
                    {"addr": "0x188", "size": 32, "decompiled": True, "hint": True},
                    {"addr": "0x1A0", "size": 16, "tier": "verified", "name": "DoOpen"},
                ],
            },
            {
                "name": "DllBattle",
                "relink_ok": True,
                "cells": [
                    {"addr": f"0x{0x180 + i * 4:X}", "tier": ["raw", "decompiled", "named", "verified"][i % 4]}
                    for i in range(40)
                ],
            },
            {
                "name": "DllTitle",
                "cells": [
                    {"addr": "384", "covered": True},
                    {"addr": "388", "decompiled": True, "namespace": "ui::title"},
                    {"addr": "392", "named": True},
                ],
            },
        ],
    }
