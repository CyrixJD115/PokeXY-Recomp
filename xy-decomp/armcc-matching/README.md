# ARMCC matching — verified byte-match decompilation

This is the real **compiler-parity breakthrough** `docs/general/findings.md`
flags as the project's outstanding blocker: compiled-vs-original matching
was stuck at 0% because nobody had the *retail* toolchain. It exists —
Pokémon X/Y's ARM11 code was built with the CTR SDK's **ARM Compiler 5
(armcc)**, not GCC, and that toolchain is available locally.

This was discovered and proven independently in a sibling project
(`~/Documents/decomp/pokemon x`, the maintainer's own from-scratch
decompilation attempt) and is migrated here on 2026-10-14 so the finding
and its proof live in this repo too. Full writeup: `docs/decomp/armcc-toolchain.md`.

## What's here

- `tools/fingerprint_fn.py` — the verifier. Compiles a candidate `.c` file
  with armcc (via Wine) and masked-word-compares it against the real
  function bytes read straight out of a plaintext `.cro` (see
  `xy-archive/src/cro.rs` — CRO modules need no decryption, just
  `xy archive exefs`/`extract-tree`/romfs to get them onto disk at
  `.Data/Raw_Extracted/romfs/`).
- `matched/DllDialogCommon/` — 14 functions proven to **masked-match** the
  retail binary word-for-word (branch immediates excepted, since those are
  link-layout facts no decompiled source can know ahead of linking):

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x8d0` | `0x2c` | 11/11 | First matched function (2026-10-13). Two calls through an unresolved import veneer. |
  | `0xcdc` | `0x34` | 13/13 | Destructor: vptr install, frees member +0x14, returns `this` (ARM C++ ABI). |
  | `0x688` | `0x3c` | 15/15 | Calls local fn @0x300 on a member, virtual call through slot 8. |
  | `0x6c4` | `0x24` | 9/9 | Virtual call through slot 2; the `r4` push is AAPCS stack-alignment padding, not a real save. |
  | `0x6e8` | `0x20` | 8/8 | Prologue-less leaf (missed by the original function catalog): a tail virtual call (`bx r1`). |
  | `0x2d8` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1` (2026-10-15). |
  | `0x2e0` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`. |
  | `0x2e8` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`. |
  | `0x2f0` | `0x4` | 1/1 | Trivial no-op void stub (single `bx lr`) — weak evidence, see note below. |
  | `0x2f4` | `0x4` | 1/1 | Trivial no-op void stub (single `bx lr`) — weak evidence. |
  | `0x2f8` | `0x4` | 1/1 | Trivial no-op void stub (single `bx lr`) — weak evidence. |
  | `0x2fc` | `0x4` | 1/1 | Trivial no-op void stub (single `bx lr`) — weak evidence. |
  | `0xd10` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |
  | `0xd18` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`. |

  The six 2026-10-15 stubs at `0x2d8`-`0x2ff` and the pair at `0xd10`-`0xd1f`
  are a run of near-identical 1-2 word leaf functions — likely default
  virtual-method overrides on the dialog-proc base class (`IsXxx() ==
  true/false`, no-op `OnXxx()`). The four single-word `bx lr` stubs are
  **weak-evidence matches**: any void function with no observable side
  effect compiles to that same one word, so the match alone doesn't prove
  the real body was empty — only that it's at most this short.

- `matched/DllNumberInput/` — 5 functions, proving the technique
  generalizes past `DllDialogCommon` to a second module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x1888` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`. |
  | `0x1a40` | `0x4` | 1/1 | Trivial no-op void stub (single `bx lr`) — weak evidence. |
  | `0x1ab0` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |
  | `0x18dc` | `0x20` | 8/8 | Copies a field into a sub-object, calls a veneer with it, stores the result. |
  | `0x1890` | `0x4c` | 19/19 | Notification dispatcher, two early-outs sharing one epilogue — see note below. |

  `0x1890` is the most instructive specimen so far: its two early-return
  paths share a single `pop {r4, pc}` epilogue with **no** `mov r0, #N`
  before it, because armcc just returns whatever is already live in r0
  at the branch site rather than re-materializing the "obvious" return
  constant. Writing both early returns as literal `return 1;` cost an
  extra instruction; matching required tracing which value was actually
  sitting in the register at each branch (0 at the null-handle check, 1
  at the already-notified check), not which value reads naturally as
  true/false. See the file's header comment for the full trace.

## Reproducing a match in this repo

```sh
# One-time: get the plaintext CRO onto disk (the game needs no AES key —
# CROs are plaintext, see docs/3ds/ + xy-archive/src/cro.rs).
./target/release/xy archive romfs .Data/Rom/X/Base/baserom_x.cci .Data/Raw_Extracted/romfs

# armcc 5.04u5 "for Nintendo" is a proprietary, shared install outside any
# repo — set ARMCC_HOME if it isn't at the tool's default
# (~/Documents/decomp/tools/armcc/ARMCC_Nintendo_5). Needs Wine.
python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
    DllDialogCommon 0x8d0 0x2c \
    xy-decomp/armcc-matching/matched/DllDialogCommon/0x8d0.c
# -> target 11 words, candidate 11 words, 11/11 match (b/bl targets masked)
# -> *** MASKED MATCH ***
```

All 5 specimens above were re-verified against this repo's own freshly
extracted `DllDialogCommon.cro` on migration day and reproduce their
masked match exactly — the toolchain and verifier are proven portable,
not a one-off result tied to the sibling repo's layout.

## What this does *not* do yet

This is per-function fingerprinting (prove a candidate's instruction
stream matches, modulo branch targets), not whole-module linking. Getting
from "5 matched functions" to "DllDialogCommon fully decompiled" needs the
remaining ~13,549 cataloged functions (project-wide: 38,385) recovered and
fingerprinted the same way, then an actual CRO relink to confirm full byte
identity including resolved branch targets. See the sibling repo's
`docs/progress.md` for the live residue-class/puzzle list (some functions
hint the *true* original compiler may be an even earlier armcc than
5.04u5 — open item, not yet closed).
