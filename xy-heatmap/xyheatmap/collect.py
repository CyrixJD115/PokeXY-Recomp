"""Read-only collectors for the project's recomp/decomp data sources.

Everything here only READS; collection produces an in-memory Dataset and a
cheap source fingerprint. Nothing writes to disk — the web UI and the
exporters consume the dataset directly.

Sources (all under a data root such as ``.Data/Source_Extracted/``):

===============================  ===========================================
``recomp/entries.c``             module table: name <-> m-number, sizes
``recomp/functions.h``           Main function starts (absolute 0x100000 frame)
``recomp/m<N>_functions.h``      per-CRO starts (module-relative 0x180 frame)
``pipeline/symbols/*.csv``       Ghidra symbol CSVs: starts, sizes, real
                                 names, namespaces (CROs pre-shifted into the
                                 module-relative frame)
``cro_extracted/<Mod>/symbols.tsv``  real export symbols (``offset`` column =
                                 module-relative); ``static/symbols.tsv`` =
                                 code.bin exports (``load_addr``, Thumb bit)
``decomp/code_c`` ``cro_c/<Mod>``  decompiled C corpus (filename prefixes;
                                 Main absolute, CROs module-relative)
``pipeline/orig`` vs ``out``     per-module SHA-256 relink check
===============================  ===========================================

Canonical frames: **Main** cells use absolute addresses (0x00100000 base);
**CRO** cells use module-relative text offsets (the 0x180 frame). Every
source above is already in one of those frames as shipped.
"""

from __future__ import annotations

import csv
import hashlib
import os
import re
from pathlib import Path

from xyheatmap.dataset import (
    FLAG_COVERED,
    FLAG_HINT,
    Cell,
    Dataset,
    Module,
)
from xyheatmap.errors import HeatmapError
from xyheatmap.palette import TIER_INDEX

_MAIN = "Main"
_GHIDRA_BASE = 0x100000

_MODULE_ROW = re.compile(r'\{"([A-Za-z0-9_]+)", &m(\d+)_base, (0x[0-9A-Fa-f]+)u, (\d+),')
_FUNC_DECL = re.compile(r"^void (?:m(\d+)_)?f_([0-9A-Fa-f]+)\(Context")
_DECOMP_NAME = re.compile(r"^([0-9A-Fa-f]{8})_")

_TIER_DECOMPILED = TIER_INDEX["decompiled"]
_TIER_NAMED = TIER_INDEX["named"]


def discover_data_root(start: Path | None = None) -> Path:
    """Walk up from ``start`` (default: cwd) looking for .Data/Source_Extracted."""
    p = (start or Path.cwd()).resolve()
    for candidate in (p, *p.parents):
        root = candidate / ".Data" / "Source_Extracted"
        if (root / "recomp" / "entries.c").is_file():
            return root
    raise HeatmapError(
        "no data root found (looked for .Data/Source_Extracted with recomp/entries.c "
        f"above {p}); pass --data-root, set XY_HEATMAP_DATA_ROOT, or supply --dataset"
    )


def _demangle_namespace(name: str) -> str:
    """Best-effort Itanium nested-name prefix -> ``a::b::Class`` ("" if unsure)."""
    if not name.startswith("_ZN"):
        return ""
    rest = name[3:]
    parts: list[str] = []
    i, n = 0, len(rest)
    while i < n:
        if rest[i] == "E":
            break
        if rest.startswith("St", i):
            parts.append("std")
            i += 2
            continue
        if parts and rest[i] in "CD" and i + 1 < n and rest[i + 1].isdigit():
            break  # constructor/destructor abbreviation ends the nested name
        j = i
        while j < n and rest[j].isdigit():
            j += 1
        if j == i:
            return ""  # substitution code or expression — not worth guessing
        try:
            ln = int(rest[i:j])
        except ValueError:
            return ""
        if ln <= 0 or j + ln > n:
            return ""
        parts.append(rest[j : j + ln])
        i = j + ln
    if len(parts) >= 2:
        return "::".join(parts[:-1])
    return ""


class _Acc:
    """Per-address accumulator while merging sources."""

    __slots__ = ("size", "name", "namespace", "tier", "flags")

    def __init__(self) -> None:
        self.size = 0
        self.name = ""
        self.namespace = ""
        self.tier = 0
        self.flags = 0


class ProjectSource:
    """The project dataset: lazily collected, fingerprinted for live reload."""

    def __init__(self, root: Path) -> None:
        self.root = Path(root)
        self.entries_c = self.root / "recomp" / "entries.c"
        if not self.entries_c.is_file():
            raise HeatmapError(
                f"data root {str(self.root)!r} is missing recomp/entries.c — "
                "this does not look like a Source_Extracted tree"
            )

    def describe(self) -> str:
        return f"project corpus at {self.root}"

    @staticmethod
    def _ci_child(parent: Path, name: str) -> Path | None:
        """parent/name, falling back to a case-insensitive match of one level.

        entries.c says ``DllEShop`` while the extracted tree says ``DllEshop``
        — the corpus is not perfectly case-consistent, so lookups tolerate it.
        """
        exact = parent / name
        if exact.exists():
            return exact
        low = name.lower()
        if parent.is_dir():
            for p in parent.iterdir():
                if p.name.lower() == low:
                    return p
        return None

    # ------------------------------------------------------------- fingerprint
    def fingerprint(self) -> str:
        h = hashlib.sha1()

        def feed(rel: str, entries: list[tuple[str, int]]) -> None:
            for name, size in sorted(entries):
                h.update(f"{rel}/{name}:{size}\n".encode())

        def feed_dir(rel: str, pattern: str) -> None:
            d = self.root / rel
            if d.is_dir():
                feed(rel, [(p.name, p.stat().st_size) for p in d.glob(pattern)])

        rec = self.root / "recomp"
        feed("recomp", [("entries.c", self.entries_c.stat().st_size)])
        feed_dir("recomp", "functions.h")
        feed_dir("recomp", "m*_functions.h")
        feed_dir("pipeline/symbols", "*.csv")
        feed_dir("cro_extracted", "*/symbols.tsv")
        feed_dir("decomp/code_c", "*.c")
        cc = self.root / "decomp" / "cro_c"
        if cc.is_dir():
            feed(
                "decomp/cro_c",
                [
                    (f"{d.name}/{p.name}", p.stat().st_size)
                    for d in cc.iterdir()
                    if d.is_dir()
                    for p in d.glob("*.c")
                ],
            )
        for rel in ("pipeline/orig", "pipeline/out"):
            d = self.root / rel
            if d.is_dir():
                feed(rel, [(p.name, p.stat().st_size) for p in d.iterdir()])
        return h.hexdigest()[:16]

    # ----------------------------------------------------------------- parsing
    def _module_table(self) -> list[tuple[str, int]]:
        """[(name, m-number)] from the entries.c module table; Main implied."""
        text = self.entries_c.read_text(encoding="utf-8", errors="replace")
        rows = [(m.group(1), int(m.group(2))) for m in _MODULE_ROW.finditer(text)]
        if not rows:
            raise HeatmapError(
                f"{self.entries_c}: no module table rows found — the file layout "
                "does not match the expected 3dsrecomp output"
            )
        return rows

    @staticmethod
    def _covered_starts(rec_dir: Path) -> dict[int, set[int]]:
        """m-number (0 = Main) -> set of covered start addresses."""
        covered: dict[int, set[int]] = {}
        main_h = rec_dir / "functions.h"
        if main_h.is_file():
            starts = covered.setdefault(0, set())
            for line in main_h.read_text(encoding="utf-8", errors="replace").splitlines():
                m = _FUNC_DECL.match(line.strip())
                if m and m.group(1) is None:
                    starts.add(int(m.group(2), 16))
        for p in sorted(rec_dir.glob("m*_functions.h")):
            mnum_match = re.fullmatch(r"m(\d+)_functions", p.stem)
            if not mnum_match:
                continue
            mnum = int(mnum_match.group(1))
            starts = covered.setdefault(mnum, set())
            for line in p.read_text(encoding="utf-8", errors="replace").splitlines():
                m = _FUNC_DECL.match(line.strip())
                if m and m.group(1) is not None and int(m.group(1)) == mnum:
                    starts.add(int(m.group(2), 16))
        return covered

    def _load_csv(self, path: Path) -> dict[int, _Acc]:
        out: dict[int, _Acc] = {}
        with path.open(newline="", encoding="utf-8") as f:
            reader = csv.reader(f)
            header = next(reader, None)
            if not header or header[0].strip('"') != "Location":
                raise HeatmapError(f"{path}: unexpected CSV header {header!r}")
            for row in reader:
                if len(row) < 4:
                    continue
                try:
                    addr = int(row[0].strip('"'), 16)
                    size = int(row[4].strip('"'), 16) if len(row) > 4 else 0
                except ValueError:
                    continue
                name, namespace = row[1], row[2]
                acc = out.setdefault(addr, _Acc())
                acc.size = max(acc.size, size)
                real = not name.startswith(("FUN_", "thunk_FUN_"))
                if real:
                    acc.name = acc.name or name
                    acc.tier = max(acc.tier, _TIER_NAMED)
                    if namespace and namespace != "Global":
                        acc.namespace = acc.namespace or namespace
                else:
                    acc.tier = max(acc.tier, _TIER_DECOMPILED)
        return out

    @staticmethod
    def _load_exports_tsv(path: Path, column: str) -> dict[int, _Acc]:
        out: dict[int, dict[str, None]] = {}
        with path.open(encoding="utf-8") as f:
            header = f.readline().rstrip("\n").split("\t")
            try:
                col = header.index(column)
                name_col = header.index("name")
            except ValueError:
                raise HeatmapError(f"{path}: unexpected TSV header {header!r}") from None
            for line in f:
                parts = line.rstrip("\n").split("\t")
                if len(parts) <= max(col, name_col) or not parts[name_col]:
                    continue
                try:
                    addr = int(parts[col], 16)
                except ValueError:
                    continue
                out.setdefault(addr, {})[parts[name_col]] = None
        accs: dict[int, _Acc] = {}
        for addr, names in out.items():
            acc = _Acc()
            acc.name = sorted(names)[0]
            acc.tier = _TIER_NAMED
            acc.namespace = _demangle_namespace(acc.name)
            accs[addr] = acc
        return accs

    @staticmethod
    def _load_imports(path: Path) -> tuple[str, ...]:
        """Dependency context: which modules this module imports from.

        ``imports.tsv`` rows: ``module<TAB>indexed<TAB>anonymous``; the
        static module appears as ``|static|`` and is reported as ``Main``.
        """
        names: list[str] = []
        with path.open(encoding="utf-8") as f:
            header = f.readline().rstrip("\n").split("\t")
            if not header or header[0] != "module":
                return ()
            for line in f:
                parts = line.rstrip("\n").split("\t")
                if not parts or not parts[0]:
                    continue
                name = "Main" if parts[0] == "|static|" else parts[0]
                names.append(name)
        return tuple(sorted(names))

    @staticmethod
    def _decomp_addrs(directory: Path) -> set[int]:
        addrs: set[int] = set()
        if not directory.is_dir():
            return addrs
        for p in directory.iterdir():
            m = _DECOMP_NAME.match(p.name)
            if m:
                addrs.add(int(m.group(1), 16))
        return addrs

    def _relink_map(self) -> dict[str, bool]:
        orig, out = self.root / "pipeline" / "orig", self.root / "pipeline" / "out"
        result: dict[str, bool] = {}
        if not orig.is_dir():
            return result
        for p in sorted(orig.iterdir()):
            other = self._ci_child(out, p.name)
            if other is None or not other.is_file():
                continue
            result[p.name.lower()] = hashlib.sha256(p.read_bytes()).hexdigest() == hashlib.sha256(
                other.read_bytes()
            ).hexdigest()
        return result

    # ----------------------------------------------------------------- collect
    def dataset(self) -> Dataset:
        warnings: list[str] = []
        table = self._module_table()  # [(name, mnum)] — raises if entries.c is wrong
        rec_dir = self.root / "recomp"
        covered = self._covered_starts(rec_dir)

        csv_dir = self.root / "pipeline" / "symbols"
        cro_dir = self.root / "cro_extracted"
        decomp_root = self.root / "decomp"
        relink = self._relink_map()

        modules: list[Module] = []
        # Main first, then the CROs in entries.c order (builder = alphabetical).
        specs: list[tuple[str, int | None]] = [(_MAIN, 0)]
        for name, mnum in table:
            if name != _MAIN:
                specs.append((name, mnum))

        for name, mnum in specs:
            is_main = mnum == 0
            accs: dict[int, _Acc] = {}

            for addr in covered.get(mnum or 0, ()):  # covered: Main=0, CRO=mnum
                accs.setdefault(addr, _Acc()).flags |= FLAG_COVERED

            csv_name = "code.bin.csv" if is_main else f"{name}.cro.csv"
            csv_path = self._ci_child(csv_dir, csv_name)
            if csv_path is not None and csv_path.is_file():
                for addr, acc in self._load_csv(csv_path).items():
                    dst = accs.setdefault(addr, _Acc())
                    dst.size = max(dst.size, acc.size)
                    dst.tier = max(dst.tier, acc.tier)
                    dst.name = dst.name or acc.name
                    dst.namespace = dst.namespace or acc.namespace
            else:
                warnings.append(f"{name}: no symbol CSV ({csv_name})")

            tsv = (
                self._ci_child(cro_dir, "static/symbols.tsv")
                if is_main
                else self._ci_child(cro_dir, name)
            )
            if tsv is not None and not is_main:
                tsv = self._ci_child(tsv, "symbols.tsv")
            if tsv is not None and tsv.is_file():
                col = "load_addr" if is_main else "offset"
                for addr, acc in self._load_exports_tsv(tsv, col).items():
                    if is_main:
                        addr &= ~1  # Thumb bit
                    dst = accs.setdefault(addr, _Acc())
                    dst.tier = max(dst.tier, acc.tier)
                    dst.name = dst.name or acc.name
                    dst.namespace = dst.namespace or acc.namespace
            else:
                warnings.append(f"{name}: no export symbols ({tsv.name})")

            dec = (
                decomp_root / "code_c"
                if is_main
                else self._ci_child(decomp_root / "cro_c", name)
            )
            decomp_addrs = self._decomp_addrs(dec) if dec is not None else set()
            if not is_main:
                # cro_c filenames carry absolute Ghidra addresses (each CRO
                # program was imported at the 0x100000 base); the canonical
                # CRO frame is module-relative.
                decomp_addrs = {a - _GHIDRA_BASE for a in decomp_addrs}
            if decomp_addrs:
                for addr in decomp_addrs:
                    acc = accs.setdefault(addr, _Acc())
                    acc.tier = max(acc.tier, _TIER_DECOMPILED)
            else:
                warnings.append(f"{name}: no decompiled C corpus")

            cells = [
                Cell(
                    addr=addr,
                    size=acc.size,
                    name=acc.name,
                    tier=acc.tier,
                    flags=acc.flags,
                    namespace=acc.namespace,
                )
                for addr, acc in sorted(accs.items())
            ]
            relink_name = "code.bin" if is_main else f"{name}.cro"
            imports: tuple[str, ...] = ()
            imp_path = (
                self._ci_child(cro_dir, "static/imports.tsv")
                if is_main
                else self._ci_child(cro_dir, name)
            )
            if imp_path is not None and not is_main:
                imp_path = self._ci_child(imp_path, "imports.tsv")
            if imp_path is not None and imp_path.is_file():
                imports = self._load_imports(imp_path)
            module = Module(
                name=name,
                cells=cells,
                relink_ok=relink.get(relink_name.lower()),
                imports=imports,
            )
            modules.append(module)

        return Dataset(
            title="Pokémon X/Y recomp + decomp progress",
            modules=modules,
            warnings=warnings,
        )


class FileDatasetSource:
    """Serves a Dataset from one generic JSON file (the standalone path)."""

    def __init__(self, path: Path) -> None:
        self.path = Path(path)
        if not self.path.is_file():
            raise HeatmapError(f"dataset file not found: {self.path}")

    def describe(self) -> str:
        return f"dataset {self.path}"

    def fingerprint(self) -> str:
        st = self.path.stat()
        return f"{st.st_size}:{st.st_mtime_ns}"

    def dataset(self) -> Dataset:
        from xyheatmap.dataset import load_dataset

        return load_dataset(self.path)
