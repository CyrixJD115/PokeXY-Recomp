"""The data model: modules of address-ordered function cells with status.

A :class:`Dataset` is a list of :class:`Module` s; a module is a list of
:class:`Cell` s (one per function in the union inventory) sorted by address.
Each cell carries:

* ``addr``   — coordinate: absolute VMA for the main binary, module-relative
               text offset for CRO modules (the collector normalizes frames)
* ``tier``   — highest status reached: raw(0) < decompiled(1) < named(2) < verified(3)
* ``flags``  — bit 0: recomp-covered, bit 1: interpreter hint (overlays)
* ``name`` / ``namespace`` / ``size`` — optional annotations

Datasets come from the project collectors (:mod:`xyheatmap.collect`) or from
a generic, documented JSON document (:func:`dataset_from_dict` /
:func:`load_dataset`), so the renderer is testable and reusable without any
project data on disk.

Generic JSON schema::

    {
      "title": "optional display title",
      "modules": [
        {"name": "DllBag", "namespace": "…", "relink_ok": true,
         "cells": [
           {"addr": "0x00000180", "size": 64, "name": "sym",
            "decompiled": true, "named": true, "verified": false,
            "covered": true, "hint": false, "namespace": "…"}
         ]}
      ]
    }

``addr`` accepts an int or a decimal/"0x"-hex string. Tiers may be given as
booleans (escalating) or a single ``"tier"`` string.
"""

from __future__ import annotations

import json
from dataclasses import dataclass, field

from xyheatmap.errors import HeatmapError
from xyheatmap.palette import TIER_INDEX

FLAG_COVERED = 1
FLAG_HINT = 2


@dataclass
class Cell:
    addr: int
    size: int = 0
    name: str = ""
    tier: int = 0
    flags: int = 0
    namespace: str = ""

    @property
    def covered(self) -> bool:
        return bool(self.flags & FLAG_COVERED)

    @property
    def hint(self) -> bool:
        return bool(self.flags & FLAG_HINT)

    def escalate(self, tier: int) -> None:
        if tier > self.tier:
            self.tier = tier

    def tip(self, tier_name: str) -> str:
        bits = [f"0x{self.addr:08X}"]
        if self.name:
            bits.append(self.name)
        meta = []
        if self.size:
            meta.append(f"{self.size} B")
        meta.append(tier_name)
        if self.covered:
            meta.append("covered")
        if self.hint:
            meta.append("interpreter hit")
        return " — ".join([bits[0], ", ".join(meta)] + ([""] if not self.name else []))


@dataclass
class Module:
    name: str
    cells: list[Cell] = field(default_factory=list)
    relink_ok: bool | None = None
    namespace: str = ""
    imports: tuple[str, ...] = ()  # modules this module imports from (dependency context)

    def __post_init__(self) -> None:
        self.cells.sort(key=lambda c: c.addr)

    def tier_counts(self) -> dict[str, int]:
        counts = {"raw": 0, "decompiled": 0, "named": 0, "verified": 0}
        for c in self.cells:
            counts[["raw", "decompiled", "named", "verified"][c.tier]] += 1
        return counts

    def covered_count(self) -> int:
        return sum(1 for c in self.cells if c.covered)

    def hint_count(self) -> int:
        return sum(1 for c in self.cells if c.hint)


@dataclass
class Dataset:
    title: str
    modules: list[Module]
    fingerprint: str = ""
    warnings: list[str] = field(default_factory=list)

    def module(self, name: str) -> Module:
        for m in self.modules:
            if m.name == name:
                return m
        low = name.lower()
        for m in self.modules:
            if m.name.lower() == low:
                return m
        raise HeatmapError(
            f"unknown module {name!r}; known modules: {', '.join(m.name for m in self.modules)}"
        )

    def total_cells(self) -> int:
        return sum(len(m.cells) for m in self.modules)

    def tier_counts(self) -> dict[str, int]:
        counts = {"raw": 0, "decompiled": 0, "named": 0, "verified": 0}
        for m in self.modules:
            for k, v in m.tier_counts().items():
                counts[k] += v
        return counts

    def covered_count(self) -> int:
        return sum(m.covered_count() for m in self.modules)

    def hint_count(self) -> int:
        return sum(m.hint_count() for m in self.modules)

    def relink_summary(self) -> tuple[int, int] | None:
        """(ok, total) over modules that carry relink data, else None."""
        known = [m for m in self.modules if m.relink_ok is not None]
        if not known:
            return None
        return sum(1 for m in known if m.relink_ok), len(known)

    def namespace_stats(self) -> dict[str, dict[str, int]]:
        stats: dict[str, dict[str, int]] = {}
        for m in self.modules:
            for c in m.cells:
                ns = c.namespace or "(no namespace)"
                s = stats.setdefault(ns, {"raw": 0, "decompiled": 0, "named": 0, "verified": 0})
                s[["raw", "decompiled", "named", "verified"][c.tier]] += 1
        return stats

    def namespace_details(self, min_tier: int = 0) -> dict[str, dict]:
        """Per-namespace tier counts + which modules the symbols live in.

        This is what makes the namespaces view explainable: a namespace is a
        subsystem name visible in named symbols (CRO exports, demangled C++
        paths, Ghidra labels), and the module set shows where that subsystem
        lives. Cells without any namespace are bucketed under "(unnamed)".
        ``min_tier`` filters which cells are aggregated.
        """
        details: dict[str, dict] = {}
        for m in self.modules:
            for c in m.cells:
                if c.tier < min_tier:
                    continue
                ns = c.namespace or "(unnamed)"
                d = details.setdefault(
                    ns, {"counts": {"raw": 0, "decompiled": 0, "named": 0, "verified": 0}, "modules": {}}
                )
                d["counts"][["raw", "decompiled", "named", "verified"][c.tier]] += 1
                d["modules"][m.name] = d["modules"].get(m.name, 0) + 1
        return details


def parse_addr(v, where: str) -> int:
    if isinstance(v, bool):  # bool is an int subclass; reject explicitly
        raise HeatmapError(f"{where}: addr must be an integer, got boolean")
    if isinstance(v, int):
        addr = v
    elif isinstance(v, str):
        try:
            addr = int(v, 16) if v.lower().startswith("0x") else int(v, 10)
        except ValueError:
            raise HeatmapError(f"{where}: cannot parse addr {v!r} as an integer") from None
    else:
        raise HeatmapError(f"{where}: addr must be an integer or hex string, got {type(v).__name__}")
    if addr < 0:
        raise HeatmapError(f"{where}: addr must be non-negative")
    return addr


def _opt_int(v, where: str) -> int:
    addr = parse_addr(v, where)
    return addr


def _escalate_from_flags(cell: Cell, spec: dict, where: str) -> None:
    tier = spec.get("tier")
    if tier is not None:
        if not isinstance(tier, str) or tier not in TIER_INDEX:
            raise HeatmapError(
                f"{where}: tier must be one of {', '.join(TIER_INDEX)}, got {tier!r}"
            )
        cell.escalate(TIER_INDEX[tier])
    for name, idx in (("decompiled", 1), ("named", 2), ("verified", 3)):
        val = spec.get(name, False)
        if not isinstance(val, bool):
            raise HeatmapError(f"{where}: {name} must be a boolean")
        if val:
            cell.escalate(idx)


def dataset_from_dict(obj, source: str = "input") -> Dataset:
    """Validate a generic dataset document; raises HeatmapError on any problem."""
    if not isinstance(obj, dict):
        raise HeatmapError(f"{source}: top level must be a JSON object")
    title = obj.get("title", "heatmap")
    if not isinstance(title, str):
        raise HeatmapError(f"{source}: title must be a string")
    raw_modules = obj.get("modules")
    if not isinstance(raw_modules, list) or not raw_modules:
        raise HeatmapError(f"{source}: 'modules' must be a non-empty list")

    modules: list[Module] = []
    for mi, raw_m in enumerate(raw_modules):
        where_m = f"{source}: module[{mi}]"
        if not isinstance(raw_m, dict):
            raise HeatmapError(f"{where_m}: must be an object")
        name = raw_m.get("name")
        if not isinstance(name, str) or not name.strip():
            raise HeatmapError(f"{where_m}: 'name' must be a non-empty string")
        mod = Module(
            name=name,
            relink_ok=raw_m.get("relink_ok") if isinstance(raw_m.get("relink_ok"), bool) else None,
            namespace=raw_m.get("namespace", "") if isinstance(raw_m.get("namespace", ""), str) else "",
        )
        raw_cells = raw_m.get("cells", [])
        if not isinstance(raw_cells, list):
            raise HeatmapError(f"{where_m}: 'cells' must be a list")
        seen: set[int] = set()
        for ci, raw_c in enumerate(raw_cells):
            where = f"{where_m} cell[{ci}]"
            if not isinstance(raw_c, dict):
                raise HeatmapError(f"{where}: must be an object")
            addr = parse_addr(raw_c.get("addr"), where)
            if addr in seen:
                raise HeatmapError(f"{where}: duplicate address 0x{addr:X} in module {name!r}")
            seen.add(addr)
            size = raw_c.get("size", 0)
            if size is None:
                size = 0
            size = _opt_int(size, f"{where} size")
            name_s = raw_c.get("name", "")
            if not isinstance(name_s, str):
                raise HeatmapError(f"{where}: name must be a string")
            ns = raw_c.get("namespace", "")
            if not isinstance(ns, str):
                raise HeatmapError(f"{where}: namespace must be a string")
            flags = 0
            if raw_c.get("covered", False) is True:
                flags |= FLAG_COVERED
            elif raw_c.get("covered", False) not in (False,):
                raise HeatmapError(f"{where}: covered must be a boolean")
            if raw_c.get("hint", False) is True:
                flags |= FLAG_HINT
            elif raw_c.get("hint", False) not in (False,):
                raise HeatmapError(f"{where}: hint must be a boolean")
            cell = Cell(addr=addr, size=size, name=name_s, flags=flags, namespace=ns)
            _escalate_from_flags(cell, raw_c, where)
            mod.cells.append(cell)
        mod.cells.sort(key=lambda c: c.addr)
        modules.append(mod)
    return Dataset(title=title, modules=modules)


def load_dataset(path: str) -> Dataset:
    try:
        with open(path, "rb") as f:
            data = f.read()
    except OSError as e:
        raise HeatmapError(f"cannot read dataset {path!r}: {e.strerror or e}") from None
    try:
        obj = json.loads(data)
    except json.JSONDecodeError as e:
        raise HeatmapError(f"dataset {path!r} is not valid JSON: line {e.lineno}: {e.msg}") from None
    return dataset_from_dict(obj, source=path)


def dataset_to_api(ds: Dataset) -> dict:
    """Compact JSON-safe projection for the web UI payload."""
    mods = []
    for m in ds.modules:
        cells = [
            [
                c.addr,
                c.size,
                c.name,
                c.tier,
                c.flags,
                c.namespace,
            ]
            for c in m.cells
        ]
        mods.append(
            {
                "name": m.name,
                "relink_ok": m.relink_ok,
                "imports": list(m.imports),
                "cells": cells,
            }
        )
    return {"title": ds.title, "fingerprint": ds.fingerprint, "modules": mods}
