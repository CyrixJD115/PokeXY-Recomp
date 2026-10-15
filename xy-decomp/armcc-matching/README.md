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

- `matched/DllNumberInput/` — 6 functions, proving the technique
  generalizes past `DllDialogCommon` to a second module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x1888` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`. |
  | `0x1a40` | `0x4` | 1/1 | Trivial no-op void stub (single `bx lr`) — weak evidence. |
  | `0x1ab0` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |
  | `0x18dc` | `0x20` | 8/8 | Copies a field into a sub-object, calls a veneer with it, stores the result. |
  | `0x1890` | `0x4c` | 19/19 | Notification dispatcher, two early-outs sharing one epilogue — see note below. |
  | `0x18fc` | `0x78` | 30/30 | Constructor; node-init tail is a moving-pointer-cursor chain, not fixed struct offsets — see note below. |

  Three more functions in this module (`0x1974`, `0x1a44`, `0x1a7c` —
  destructor-stage helpers that install a dtor vtable and conditionally
  free one member) were attempted and reconstructed correctly in
  semantics, but hit the **same vptr-install scheduling residue class**
  already documented for `DllDialogCommon`'s `0xc34`/`0xc68`/`0xca4`:
  armcc 5.04u5 always schedules the vtable pool-load register-first,
  while the retail binary schedules the member load first. Confirms the
  residue class is a toolchain-version property, not a per-function
  fluke — see `docs/decomp/armcc-toolchain.md`.

  `0x1890` is the most instructive specimen so far: its two early-return
  paths share a single `pop {r4, pc}` epilogue with **no** `mov r0, #N`
  before it, because armcc just returns whatever is already live in r0
  at the branch site rather than re-materializing the "obvious" return
  constant. Writing both early returns as literal `return 1;` cost an
  extra instruction; matching required tracing which value was actually
  sitting in the register at each branch (0 at the null-handle check, 1
  at the already-notified check), not which value reads naturally as
  true/false. See the file's header comment for the full trace.

  `0x18fc` is a constructor whose node-initialization tail is written in
  the retail binary as a **sequence of moving-pointer-cursor stores**
  (`str r6,[r0,#92]!` / `add r0,r0,#4` / `str r6,[r0,#-84]!` / `str r5,
  [r0,#-12]!` — each a pre-indexed store that writes and advances the
  pointer in one instruction) rather than fixed-offset struct-member
  access off one stable base. Translating it the same way in C
  (`p += 0x5c; *p = 0; p += 4; p = fn(p); p -= 0x54; *p = 0; p -= 0xc;
  *p = parent;`) matched on the first try. This is the sibling pattern
  to `DllDialogCommon@0x710` (same overall constructor shape) — but that
  specimen's node-init tail uses one `sub` followed by fixed offsets
  instead of a continued cursor chain, and still doesn't match; the two
  specimens turned out to use genuinely different source idioms rather
  than one rule explaining both. See `docs/decomp/armcc-toolchain.md`.

- `matched/DllStrInput/` — 3 functions from a **third** module, confirming
  the trivial-stub/simple-accessor cluster recurs near the tail of every
  Dll module built on the shared dialog-proc base class:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x6148` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`. |
  | `0x666c` | `0x8` | 2/2 | Trivial byte-field setter, void return. |
  | `0x6674` | `0x14` | 5/5 | Boolean getter through one pointer indirection. |

  This module also has three more `0x1974`-style destructor-stage
  functions (around `0x6298`, `0x6600`, `0x6638`) that were recognized
  as the same already-documented vptr-install-scheduling residue class
  on sight and skipped rather than re-attempted.

- `matched/DllFatalErrorProc/` — 1 function, a **fourth** module, this
  one unrelated to the `Dll*Input` family (fatal-error dialog proc):

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x2bc` | `0x34` | 13/13 | Destructor: vptr install, free member @+0x14, returns `this`. |

  Structurally byte-identical in shape to `DllDialogCommon@0xcdc` (same
  member offset `0x14`, same literal-pool vtable load, same `bl` to a
  masked free import) — confirms this dtor idiom is a shared-base-class
  pattern reused verbatim across unrelated Dll modules, not a coincidence
  tied to the dialog-input family. The module's neighboring `0x284`
  function has the same prologue/body but tail-calls out (`pop {r4,lr};
  b <import>`) instead of `pop {r4,pc}` — not attempted here, flagged as
  a plausible next easy win for this module.

- `matched/DllLangSelect/` — 1 function, a **fifth** module (language
  select menu, unrelated to the `Dll*Input` family):

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x5b0` | `0x1c` | 7/7 | Conditional accessor: veneer call gates between a member load and a re-materialized constant. |

  Real functions in this module start at `0x438` (the import-veneer
  table runs `0x180`-`0x437`, 0x1b4c bytes total for the `.text`
  segment at file offset `0x180`, per the CRO segment table). `0x5b0`
  is notable for *not* reusing the veneer's own boolean result on the
  false branch: `cmp r0,#0; ldrne r0,[r4,#20]; moveq r0,#2` throws the
  call's return value away entirely once it's known nonzero/zero and
  re-materializes the literal `2` — matching pattern #6 from this
  toolchain's residue notes (don't assume the "obvious" value is what's
  actually live in the register).

### Tool addition: `--mask-ranges` for jump-table data (2026-10-15)

Some larger dispatcher functions (`DllDialogCommon@0x300`/`0xdd8`,
`DllNumberInput@0x1af8`, ...) compile a `switch` into a jump table: a
run of raw address words immediately after an `ldrcc pc,[pc,rX,lsl#2]`
instruction. In the unlinked CRO those words are zero placeholders (the
real addresses are a link-time fact), so they can never byte-match an
isolated compile. `fingerprint_fn.py` now takes `--mask-ranges
offset:length[,offset:length...]` (byte ranges relative to the
function's own start) to exclude exactly those words from comparison.

**Important limitation, not yet resolved:** masking the table proves the
function has the right *number* of case-body blocks, each byte-correct
in the right physical order — it does **not** prove which case *value*
(0, 1, 2, ...) selects which block, because that mapping is exactly what
the masked table encodes. Any case-number assignment in a candidate that
uses this flag is inferred from physical body order (on the assumption
armcc emits bodies in ascending case-value order) and must be documented
as a hypothesis, never asserted as fact. The tool prints this caveat at
runtime whenever `--mask-ranges` is used.

First attempt with it, `DllDialogCommon@0xdd8` (a 5-case property
setter/getter switch with no function calls at all), did not reach a
full match: the retail binary has a `push {r4, lr}` / `pop {r4, pc}`
frame the candidate didn't need (no calls in my reconstruction to
justify saving `lr`), and the default case's three sequential pool
loads are interleaved with `e320f000` NOP-hint words in the retail
binary — the project's own marker for post-link branch-relaxation nops
— suggesting the real default case made calls that got relaxed away
entirely, not just argument-setup loads. Left as an open item; no
case-to-block mapping was ever asserted for it since it never matched.

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
