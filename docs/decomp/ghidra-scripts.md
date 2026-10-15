# Running the 3DS Ghidra script suites under Ghidra 12

Ghidra 12 **removed Jython**, so the stock `analyzeHeadless` refuses `.py`
scripts ("Ghidra was not started with PyGhidra"). The 3DS script ecosystem
([zaksabeast/3ds-Ghidra-Scripts](https://github.com/zaksabeast/3ds-Ghidra-Scripts),
[AlgebraManiacABC's exporter](https://github.com/AlgebraManiacABC/3ds-Ghidra-Scripts-Java))
was written for Jython — two problems: getting CPython scripts running at
all, and the Jython-era idioms inside them.

## 1. Install PyGhidra

```bash
pip install pyghidra          # pulls jpype; needs JDK 21+ and GHIDRA_INSTALL_DIR
export GHIDRA_INSTALL_DIR=/opt/ghidra
```

## 2. Drive headless through PyGhidra, not the CLI

`xy-decomp/ghidra/xy_ghidra_3ds.py` in this repo starts the JVM via `pyghidra.start()`
and runs GhidraScripts through `pyghidra.ghidra_script()` — which supports
any script type *except* that Java scripts need bundle registration, so use
the Python exporter (`xy-decomp/ghidra/ExportSymbolsHeadless.py`).

```bash
# configure once (or edit the env defaults at the top of the script)
export XY_PROJECT_LOC=/path/to/ghidra_project_dir
export XY_PROJECT=PokemonX
python3 xy-decomp/ghidra/xy_ghidra_3ds.py code.bin       # one program
python3 xy-decomp/ghidra/xy_ghidra_3ds.py --all          # every program in the project
```

## 3. Patch the upstream scripts for CPython/headless

Fetch the upstream suite, apply these fixes (all small):

1. **`ghidra_utils.py` byte helpers** — `array('b', ''.join(map(chr, …)))`
   is Jython-only. Use `bytes(b & 0xff for b in bs)`; `readBytes` must pass
   a JPype `JArray(JByte)` buffer to `memory.getBytes` (the
   `getBytes(addr, int)` overload Jython provided does not exist).
2. **Headless dialogs** — `Label3dsSVCs.py` calls `askYesNo`; define a
   module-level `def askYesNo(...): return False` so SVC sites get
   comment+bookmark instead of stalling headless.
3. **`Label3dsHandles.py` raises** when `ctr::srv::GetServiceHandleDirect`
   is unlabeled. XY uses the C-era `srvGetServiceHandle` API — the anchor
   doesn't exist. Replace the `raise` with a printed skip so the rest of
   the postScript chain survives.
4. **`Set3dsTlsType.py`** — wrap each TLS site in try/except; the
   decompiler NPEs on some functions and one bad site killed the whole run.
5. **`ctr_services.py`** — `services.keys() + services_command_only.keys()`
   is Python-2 dict concatenation: wrap in `list(...)`; and pass
   `.encode('ascii')` to `findBytes` (Jython auto-converted str→byte[]).
6. **Shadowed builtin** — don't name parameters `bytes` if the body calls
   `bytes(...)`.

## 4. What you get on Pokémon X

- 831 SVC sites bookmarked + pre-commented in `code.bin`
- thread-local storage retyped and renamed (`tls`)
- IPC/handle attribution limited: XY predates the C++ `srv` API the handle
  scripts anchor on (see #3)
- `ExportSymbolsHeadless.py` writes the pipeline-format CSVs
  (`Location,Name,Namespace,Mode,Size,Segment`) for every program in the
  project — see [../general/findings.md](../general/findings.md) for the
  coordinate-frame and thunk-dedup gotchas before feeding them to
  3DS-Decomp-Pipeline.

## 5. Gotcha: the Ghidra project location can't live under a dot-prefixed directory

2026-10-15, rebuilding `.Data/Source_Extracted` from scratch on a fresh
machine: `analyzeHeadless .Data/Source_Extracted/decomp/ghidra_project
PokemonX -import …` aborted immediately with `Abort due to Headless
analyzer error: Path element starting with '.' is not permitted`
(`NamingUtilities.checkName` via `ProjectLocator`), even though the path
was already absolute (AGENTS.md's existing "requires absolute paths"
note is about a *different* failure mode — a truly relative path — and
doesn't cover this one).

Ghidra's `ProjectLocator`/`NamingUtilities` reject **any** path element
that starts with `.`, not just a leading `.`/`..` for "relative path" —
so `.../.Data/Source_Extracted/decomp/ghidra_project` fails purely
because `.Data` is dot-prefixed, regardless of what comes after it or
whether the whole path is absolute. This directly conflicts with this
repo's own dot-prefixed-top-level-dir convention
(`.Data/`/`.thirdparty/` = machine-local workspace material, see
AGENTS.md's Repo layout section) for exactly the one artifact — the
Ghidra project's `.gpr`/`.rep` store — that convention says should live
there.

**Workaround:** point the `<project_location>` argument of
`analyzeHeadless` at any path with no dot-prefixed element (a plain
`ghidra_project/` directory, repo-root or elsewhere, gitignored same as
`.Data/`) — only the *project store* needs to move; input files
(`code.bin`, CRO `.bin`s) and the `-postScript` output directory are
ordinary CLI string arguments, not `ProjectLocator` paths, and can still
live anywhere under `.Data/Source_Extracted/` without issue. Confirmed:
moving only `PROJ` out from under `.Data/` let both the `code.bin` import
and all 133 CRO module imports (`xy-heatmap/shell/rebuild_v2.sh` stages
C1/C2 logic) complete normally, writing their decompiled C back into
`.Data/Source_Extracted/decomp/{code_c,cro_c}/` as usual.
