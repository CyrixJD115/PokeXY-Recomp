# Pokemon X — Decomp / Native-Port Research Report

*Furyx, Oct 10 2026. All links live as of today. Legality note: everything below assumes your own dumped copy — decomp outputs, symbol maps and recomp libraries derived from game code must not be shared.*

---

## TL;DR

- A **generic 3DS static recompiler exists now**: `fearkov/3dsrecomp` + the `Zakuro` runtime (released Sept 25, 2026, actively committed daily). ARM11 → C, ~10 min per game, 19 titles tested / 17 playable. This is the closest thing to "an engine that takes 3DS games natively" and the right vehicle for your goal.
- A **matching-decomp build pipeline exists**: `AlgebraManiacABC/3DS-Decomp-Pipeline` (Ghidra symbol CSVs → split → compile → byte-compare → relink). That's the "proper" long-haul route.
- Pokemon X is a **good but non-trivial target**: the game is layered (ARM code + CRO dynamic modules + AMX scripts + GARC data). Version pinning (v1.0 vs v1.5) breaks naive tooling — details below.
- Pokemon X has **no public decomp project** (unlike Super Mario 3D Land / Mario Kart 7). If you want "perfect," you're pioneering; the tooling above is the toolkit for it.

---

## 1. The landscape (what actually exists)

### Static recompilation (fast path to "native")
| Tool | What it does | Status |
|---|---|---|
| [fearkov/3dsrecomp](https://github.com/fearkov/3dsrecomp) | Static recompiler: finds functions in `code.bin` + CROs, emits C, compiles to a library. ARM/Thumb/VFP. Interpreter fallback for gaps. C override files for hand-fixes. Can link a **standalone program**. | Created 2026-09-25, 59 commits, updated daily, Rust, Win+Linux |
| [fearkov/Zakuro](https://github.com/fearkov/zakuro) (companion runtime) | The HLE runtime the recompiled library runs under: 3DS services, graphics, audio, input. Recompile button built in. | Experimental, 19 titles tested, 17 playable (Monster Hunter XX + FE Fates rough). **Pokemon X untested — open opportunity** |

Requirements: Rust, GCC or Clang, your decrypted dump. Recompile ≈ 10 min/game.

### Matching decompilation (the "perfect source" route)
| Tool | What it does |
|---|---|
| [AlgebraManiacABC/3DS-Decomp-Pipeline](https://github.com/AlgebraManiacABC/3DS-Decomp-Pipeline) | Full pipeline: splits `code.bin` **and CRO modules** into per-symbol objects from Ghidra-exported CSV symbol lists, compiles decompiled sources with matching compiler, byte-compares (relocation-aware bitmasking), links back to a SHA-256-identical binary. Needs devkitARM. |
| [RE-Pepper](https://github.com/RE-Pepper/RE-Pepper) / [RedPepper](https://github.com/3dsdecomp/RedPepper) | Super Mario 3D Land EU decomp — proof a 3DS matching decomp ships. Study their repo layout. |
| [strayfodanator/DashDecomp](https://github.com/strayfodanator/DashDecomp) | Mario Kart 7, explicitly aiming for a native PC port. The other working example. |
| [kynex7510/3ds_ida](https://github.com/kynex7510/3ds_ida) | IDA loader for `code.bin` / ExeFS / CXI + scripts: `find_syscalls.py`, `fix_decomp.py`, `setup_switches.py`, `make_ips_patch.py` (IPS straight into Luma). Capstone 5.0.3. |
| [zaksabeast/3ds-Ghidra-Scripts](https://github.com/zaksabeast/3ds-Ghidra-Scripts) | Ghidra 10/11 suite, run in order: `Label3dsSVCs.py` → `Label3dsHandles.py` → `Label3dsCTRFns.py` → `Set3dsTlsType.py`. Auto-labels kernel calls, service handles, IPC, TLS. This is what turns Ghidra soup into readable code. |

### Pokemon X/Y specific (data + script layer — no ARM needed)
| Tool | What it does |
|---|---|
| [BlastSlimey/XAMX](https://github.com/BlastSlimey/XAMX) | Dis/re-assembler for the **AMX scripts** in XY/ORAS/SM/USUM + docs. Huge: a large share of game logic (events, overworld, battle glue) lives in these scripts, not in ARM code. |
| [kwsch/pk3DS](https://github.com/kwsch/pk3DS) | ExeFS/RomFS editor + randomizer. Correct ExeFS unpacker (other tools mangle filenames). |
| [projectpokemon.org Gen 6 docs](https://projectpokemon.org/home/docs/gen-6/pkm-structure-xy-r66/) | GARC lists, PKM structure, save structure, `pml::pokepara::Accessor` internals. |
| [smealum's XY model parser](https://gist.github.com/smealum/9394125) | BCH model/texture parsing reference. |

---

## 2. Know your target: Pokemon X binary anatomy

Four layers — "decomping the game" means deciding which layers you touch:

1. **`code.bin`** (ExeFS `.code`, LZSS-compressed): the ARM11 main executable. XY v1.0 vs **v1.5 (latest update): 5,169,152 bytes decompressed**.
2. **CRO modules** (`DllStartMenu.cro` etc.): GameFreak's dynamic libraries, loaded at runtime from `rom:/` (base) or `rom2:/` (update). XY is CRO-heavy — any pipeline must parse CRO (both 3dsrecomp and the pipeline above do).
3. **AMX scripts**: bytecode for game logic, editable with XAMX without ever touching ARM.
4. **GARC data** (RomFS): Pokemon stats, moves, models, text. Documented; pk3DS edits these.

**Version gotchas (this is probably why your decomp is "not perfect"):**
- v1.0 `code.bin` ≠ v1.5 `code.bin` — different offsets/size. Mixing base code with update data crashes or silently misbehaves.
- pk3DS Mart/Tutor editors specifically break on v1.5; most other editors are version-agnostic. Known issue, byte-pattern fix documented in [pk3DS#147](https://github.com/kwsch/pk3DS/issues/147) (incl. known offsets: shiny rate XY v1.0 `0x0004F464` vs v1.5 `0x0004F740`).
- Extract **v1.5** `code.bin` from the update title via GodMode9: `0004000e/00055d00` (X) / `00055e00` (Y) → content → `.app` → NCCH options → mount → `exefs/.code` → copy out, rename to `code.bin`. GM9's "Extract .code" decompresses LZSS for you; manual: `ctrtool -t lzss --lzssout=code.bin .code`.
- Pick ONE version for the whole project and write it on the wall. For a native port, v1.5 is the right base (it's what a cart + update actually runs).

---

## 3. Routes ranked (pick per goal)

### Route A — Play + mod (hours, zero ARM)
Azahar (merged Citra successor, active) + your dump → pk3DS + XAMX for edits. Satisfies "make the game do what I want" for most definitions. **Do this in parallel regardless** — it's your reference implementation for verifying recomp/decomp behavior against.

### Route B — Static recomp to native (days, the new hotness)
1. Rust + GCC/Clang installed.
2. Build [Zakuro](https://github.com/fearkov/zakuro), feed it your decrypted XY dump, hit recompile.
3. Pokemon X isn't in the confirmed 19 — you may hit gaps (that's what the interpreter fallback + C overrides are for). Report gaps upstream; the project is 2 weeks old and hungry for exactly this.
4. End state: game logic as native compiled C, HLE runtime for services. Not yet a standalone .exe, but the linker option exists for pure cases.

### Route C — Ghidra/IDA RE + matching decomp (months, "perfect source")
1. Get v1.5 `code.bin` (above) + the CROs from RomFS.
2. Ghidra: load as ARM:LE:32:v7, run auto-analysis, then the 3ds-Ghidra-Scripts chain. (IDA alternative: 3ds_ida loader.)
3. Export symbol CSVs as you name functions → feed `3DS-Decomp-Pipeline` → objdiff loop until objects match.
4. XY has no public symbol map — you'll bootstrap from CRO export tables, RTTI class names (GameFreak ships C++ RTTI — gold), and string references. `pml::pokepara::Accessor` etc. are already documented and give you anchor symbols.
5. Realistic: engine skeleton in weeks of evenings, "playable from source" is a community-scale project.

### Route D — Full WWHD-style native port (solo: borderline)
Auto-recomp + hand-written CTR SDK shim + PICA→Vulkan. 3dsrecomp/Zakuro already IS steps 1–2; route D = replacing Zakuro's runtime with standalone reimplementations. Only worth it if B works and you want to cut the runtime out. Watch Zakuro's maturity first.

**My recommendation:** B now, C opportunistically (name functions as you read them in B's output — they're the same addresses), A for playing tonight.

---

## 4. Why your current decomp is probably "not perfect"

Likely one of:
- **Version mismatch**: decomped v1.0 while running/patching v1.5 (or vice versa). Fix: re-extract matching the update actually installed.
- **Skipped LZSS**: analyzing the compressed `.code` instead of decompressed `code.bin` (GM9 extracted pre-decompression, or tool read raw). Check: decompressed v1.5 = exactly 5,169,152 bytes.
- **Only `code.bin`, no CROs**: the interesting GameFreak logic is heavily CRO-side. Pull `Dll*.cro` from RomFS too.
- **Ghidra with no scripts**: raw auto-analysis on ARM11+syscalls reads terribly. The 3ds-Ghidra-Scripts chain is the difference between soup and structure.
- **AMX layer untouched**: if your goal is behavior edits, ARM decomp is the wrong layer entirely — XAMX scripts are where it's at.

Send me the file you've got (or its size + first bytes) and I'll tell you which case you're in.

---

## 5. Toolbox summary

- Dump: 3DS with Luma3DS + GodMode9 (you're doing the weekend console pass anyway)
- Decompress: GodMode9 "Extract .code" or `ctrtool -t lzss`
- Read/edit data: pk3DS, XAMX, projectpokemon docs
- Disassemble/decompile: Ghidra + zaksabeast scripts (or IDA + 3ds_ida)
- Native recomp: 3dsrecomp + Zakuro
- Matching decomp build: 3DS-Decomp-Pipeline + devkitARM + objdiff
- Play/reference: Azahar
