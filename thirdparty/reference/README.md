# thirdparty/reference/ — upstream reference tools (gitignored)

Reference-only clones and binaries: the upstream implementations we ported
from, study, or run occasionally — none of them are executed by this repo's
build loops (those live in `thirdparty/` proper: 3dsrecomp, zakuro,
3DS-Decomp-Pipeline, objdiff). Original context:
`docs/PokemonX_Decomport_Report.md` (the research report that seeded this
toolchain). All are re-obtainable git clones/official binaries; pinned to
whatever was `main` on 2026-10-10.

| Folder | Stage |
|---|---|
| `1-emulation/` | azahar (Citra-successor emulator source — HLE reference + runtime oracle) |
| `2-ctr-formats/` | Project_CTR (ctrtool + makerom source), 3dstool, makerom binary — ROM repack path |
| `3-xy-data/` | pk3DS, XAMX, Ohana3DS ×2, smealum BCH gist — the XY data-format references |
| `5-decomp/` | 3ds-Ghidra-Scripts (+Java companion incl. the RTTI suite), nnheaders, RE-Pepper |

## Per-tool notes worth keeping

- **azahar** — [azahar-emu/azahar](https://github.com/azahar-emu/azahar). Debugger is the runtime oracle for recomp/decomp behavior questions.
- **Project_CTR** — [3DSGuy/Project_CTR](https://github.com/3DSGuy/Project_CTR). ctrtool decompresses ExeFS: `ctrtool -t lzss --lzssout=code.bin .code`. makerom is the CIA/NCCH builder for any future repack.
- **3dstool** — [dnasdw/3dstool](https://github.com/dnasdw/3dstool), prebuilt in `bin/`.
- **pk3DS** — [kwsch/pk3DS](https://github.com/kwsch/pk3DS). The ExeFS/RomFS editor; **our source of truth for text-bank tables** (`TextReference.GameText_XY`) and GARC/LZSS semantics (both ported into `xy`). ⚠️ Mart/Tutor editors break on v1.5 (pk3DS#147; shiny-rate offsets: v1.0 `0x0004F464`, v1.5 `0x0004F740`).
- **XAMX** — [BlastSlimey/XAMX](https://github.com/BlastSlimey/XAMX). AMX dis/re-assembler; XY's 59 global scripts round-trip byte-identically through it (outputs in `Data/src_extracted/xamx/`).
- **Ohana3DS-Rebirth (+Fork)** — BCH/CGFX model/texture reference implementations.
- **smealum-bch-parser** — [gist 9394125](https://gist.github.com/smealum/9394125), minimal BCH reference.
- **3ds-Ghidra-Scripts** — [zaksabeast](https://github.com/zaksabeast/3ds-Ghidra-Scripts). Upstream of our **patched working copies** in `scripts/ghidra/` (run order: Label3dsSVCs → Label3dsHandles → Label3dsCTRFns → Set3dsTlsType; our patches = CPython byte helpers, headless-safe dialogs, graceful handle-skip).
- **3ds-Ghidra-Scripts-Java** — [AlgebraManiacABC](https://github.com/AlgebraManiacABC/3ds-Ghidra-Scripts-Java). Companion scripts for the decomp pipeline; **the RTTI suite (ParseRTTITypeInfo / DemangleRTTIString / CreateTypeInfoStructs / AutoFillProgramClasses) is the main unexplored lever for naming the 38k functions**.
- **nnheaders** — [mk7re](https://github.com/mk7re/nnheaders). Community NNSDK headers — type SDK calls in Ghidra; useful when writing C overrides.
- **RE-Pepper** — [RE-Pepper](https://github.com/RE-Pepper/RE-Pepper). Super Mario 3D Land EU matching decomp — proof a 3DS matching decomp ships; study its repo layout. (Windows-oriented; reference only.)

## Deleted 2026-10-11 (not needed; re-obtainable if ever wanted)

- **kuriimu2** v1.2.1 Linux binaries — GARC4 plugin rejects XY's v4 GARCs, needs `DOTNET_ROLL_FORWARD=LatestMajor`, mangles `/mnt/...` paths. [Re-download](https://github.com/FanTranslatorsInternational/Kuriimu2/releases).
- **3ds_ida** — IDA loader/scripts; this project is Ghidra-based. [kynex7510/3ds_ida](https://github.com/kynex7510/3ds_ida).
- **MK7-Memory** — Mario Kart 7 structure docs (wrong project; kept as inspiration only — the pattern to copy for documenting XY structures). [mk7re/MK7-Memory](https://github.com/mk7re/MK7-Memory).

## External references (never cloned)

- **projectpokemon.org Gen 6 docs** — GARC lists, PKM/save structure, `pml::pokepara::Accessor` — bootstrap symbol anchors.
- **Ghidra** 12.1.2 at `/opt/ghidra` (Jython removed — use `scripts/ghidra/xy_ghidra_3ds.py`, the PyGhidra driver; pyghidra 3.1.0 + jpype 1.7.1 in user site-packages).
- **devkitARM** — not installed; Arch `arm-none-eabi-gcc/binutils/newlib` covers the pipeline (symlinked from `Data/src_extracted/pipeline/tools/`).
- **GodMode9 + Luma3DS** — on the console; v1.5 update `code.bin` comes from `0004000e/00055d00` (X) / `00055e00` (Y) → see `Data/update/README.md`.

## Version rule

The project is pinned to **v1.0** (X code.bin = 5,156,864 B). Do not mix
base code with v1.5 update data — always decompressed `code.bin`, CROs
included (the interesting GameFreak logic is CRO-heavy). The v1.5 slot and
its own checklist live in `Data/update/README.md`. Derived outputs must
never be shared.
