# The retail toolchain is ARM Compiler 5 (armcc), not GCC

## 2026-10-14 — compiler-parity blocker identified and partially cracked

`docs/general/findings.md` has long flagged "compiler parity (matching
GCC version/flags)" as the prerequisite for any real matching-decomp
progress, without which compiled-vs-original matching stays at 0% across
all 134 binaries (code.bin + 133 CROs). That framing had a wrong
assumption baked in: **the retail game was never built with GCC.**

Pokémon X/Y's ARM11 userland code was built with the CTR SDK's **ARM
Compiler 5 (armcc)** — the same RVCT-family toolchain Nintendo licensed
for 3DS first-party and licensed-developer titles. A real, usable copy is
sitting at `~/Documents/decomp/tools/armcc/ARMCC_Nintendo_5`: confirmed
**ARM Compiler 5.04u5 "for Nintendo"** (version strings in its own
readmes; `bin/` has `armcc.exe armlink.exe armasm.exe fromelf.exe
armar.exe` — the full RVCT 5 toolchain, Windows binaries, run through
Wine on Linux). `GCC` was simply the wrong toolchain family to be chasing
compiler parity with — any amount of GCC flag-tuning could never close
this gap, since GCC's codegen for a given C function never matches
armcc's instruction selection choices exactly (see the `strd`-vs-`stm`
residue class below).

**Provenance:** this was discovered and proven in a sibling project
(`~/Documents/decomp/pokemon x`, the maintainer's own independent,
from-scratch decompilation attempt — a different repo, different
tooling: no Ghidra, no `xy-archive`, its own CRO parser and disassembler).
Migrated into this repo on 2026-10-14 so the finding and a working proof
live here too — see `xy-decomp/armcc-matching/README.md` for what was
copied over and how to reproduce it.

### Setup

- `ARMCC_HOME` = `~/Documents/decomp/tools/armcc/ARMCC_Nintendo_5`
  (default the ported tooling assumes; override via env var). **Never
  commit this** — proprietary, Nintendo-licensed, same handling as ROM
  dumps and key material under `.Data/`.
- License: `<ARMCC_HOME>/../ARMCC_License/5.x/nintendo-anyhost.lic`
  (FlexLM, `ARMLMD_LICENSE_FILE`).
- Invocation: `wine <ARMCC_HOME>/bin/armcc.exe -c <flags> -o out.o in.c`
  — straightforward; no installer needed beyond having Wine, the compiler
  files extract/run as-is.
- Object inspection: `arm-none-eabi-objcopy -O binary --only-section=.text`
  pulls the raw instruction bytes back out for comparison (devkitARM's
  `objcopy` understands armcc's ELF output fine; no need for armcc's own
  `fromelf`).

### Fingerprinted flags

`--cpu=MPCore --arm --apcs=/interwork -O2 -Otime` — ARM11 MPCore target,
ARM (not Thumb) mode, interworking, optimized for time. Found by sweeping
flag combinations against the first specimen (`DllDialogCommon@0x8d0`)
until the output matched. `-O3` produces identical output on every
specimen checked so far, but `-O2` is kept as the default because `-O3`
licenses cross-function inlining that would hurt per-function matching
once whole modules get compiled together rather than one function at a
time.

### Match criterion: masked words, not raw bytes

A function's compiled bytes can match the retail original in every way
*except* for `b`/`bl` branch-immediate fields, which encode the target's
final linked address/offset — a fact of where the linker placed
everything, not something an isolated single-function compile can know.
The verifier (`xy-decomp/armcc-matching/tools/fingerprint_fn.py`, ported
from the sibling repo's `tools/fingerprint_fn.py`) masks those 24-bit
immediates before comparing (keeps the condition/opcode/link bits) and
calls it a **masked match**. A function that masked-matches will
byte-match for real once it's recompiled in place inside its real module
and linked against its real call targets — the remaining work moves from
"is this function even close" to "does the linker reproduce the branch
immediates too" (a `3DS-Decomp-Pipeline` / CRO-relink problem, already
proven exact on original object code — see `docs/general/findings.md`'s
134/134 relink note).

### 5 functions masked-matched (DllDialogCommon)

| Offset | Size | Words matched | What it does |
|---|---|---|---|
| `0x8d0` | `0x2c` | 11/11 | Two calls through an unresolved import veneer (first match, 2026-10-13). |
| `0xcdc` | `0x34` | 13/13 | Destructor: installs vptr, frees member `+0x14`, returns `this` (ARM C++ ABI convention — destructors return the object pointer). |
| `0x688` | `0x3c` | 15/15 | Calls a local function at `0x300` on a member, then a virtual call through vtable slot 8. |
| `0x6c4` | `0x24` | 9/9 | Virtual call through vtable slot 2; the `r4` push in the prologue is AAPCS stack-alignment padding, not a real register save. |
| `0x6e8` | `0x20` | 8/8 | A prologue-less leaf function the original function-boundary catalog missed: a tail virtual call (`bx r1`). |

Re-verified against this repo's own freshly extracted
`DllDialogCommon.cro` (`.Data/Raw_Extracted/romfs/DllDialogCommon.cro`,
plaintext per `xy-archive/src/cro.rs` — no decryption needed) on migration
day; all 5 reproduce their masked match exactly here, confirming the
toolchain and verifier are genuinely portable rather than an artifact of
the sibling repo's specific layout.

### Open: the *real* original compiler may be even older

Several uncommitted residue classes in the sibling repo's catalog are
**consistent 1-3-word mismatches**, not noise — strong evidence of
version skew rather than wrong source:

- `r1`-zero patterns around allocator/inlined-destructor contexts
  (`0x5e8`, `0x638`, `0x7a4`, `0x7f0`)
- vptr-install instruction scheduling (`0xc34`, `0xc68`, `0xca4`)
- branch polarity in switch-statement lowering (`0x840`)
- **`strd` vs `stm` instruction selection at `0xdac`** — this one is hard
  evidence: 5.04u5 picks `strd` where the retail binary has `stm` for the
  same store pattern, a codegen choice that differs between armcc
  releases, not something any flag toggles. Points at hunting an earlier
  armcc (CTR SDK era, plausibly 4.1 or an early 5.0x) to close these.

Not yet investigated in this repo; tracked as an open item here and in
the sibling repo's `docs/progress.md`.

### Dead end avoided (now corrected)

The earlier framing — "compiler parity (matching **GCC** version/flags)"
— was simply the wrong toolchain family. `xy-recomp`'s GCC-based builds
(devkitARM, static recompilation) remain the right tool for *that* track
(interpreting/recompiling to a portable C target doesn't need to
byte-match the original compiler); it's specifically **matching-decomp's
byte-identical goal** that needs armcc, not GCC. `docs/general/findings.md`
has been updated to point here instead of repeating the GCC assumption.
