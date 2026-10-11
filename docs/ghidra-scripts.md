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

`ghidra/xy_ghidra_3ds.py` in this repo starts the JVM via `pyghidra.start()`
and runs GhidraScripts through `pyghidra.ghidra_script()` — which supports
any script type *except* that Java scripts need bundle registration, so use
the Python exporter (`ghidra/ExportSymbolsHeadless.py`).

```bash
# configure once (or edit the env defaults at the top of the script)
export XY_PROJECT_LOC=/path/to/ghidra_project_dir
export XY_PROJECT=PokemonX
python3 ghidra/xy_ghidra_3ds.py code.bin       # one program
python3 ghidra/xy_ghidra_3ds.py --all          # every program in the project
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
  project — see [findings.md](findings.md) for the coordinate-frame and
  thunk-dedup gotchas before feeding them to 3DS-Decomp-Pipeline.
