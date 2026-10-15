# The retail toolchain is ARM Compiler 5 (armcc), not GCC

## 2026-10-15 — armcc-matching progress wired into the heatmap

`xy-heatmap/heatmap.py`'s dashboard/treemap previously only tracked the
Ghidra/3dsrecomp corpus (breadth: identified/decompiled/recomp-covered),
with no visibility into this track's depth (byte-exact proof). Added a new
status bit (32, "armcc masked-matched") alongside the existing
`identified/decompiled/verified/hint/recomp` bits:

- `xhmap/collect.py`'s `parse_armcc_matched()` auto-discovers
  `xy-decomp/armcc-matching/matched/<Module>/<offset>.c(pp)` — no flag or
  manual JSON needed (unlike bit 4 "Natively Verified", which still waits
  on a real `--verified` producer). Lookup is case-insensitive against
  `entries.c`'s module names (hit a real bug during validation: `DllEshop`
  in `matched/` wasn't matching `entries.c`'s `DllEShop` and silently
  dropped that module's one match from the totals — same case-spelling
  gotcha already documented for `resolve_ci`).
- Offsets are already in the same module-relative `.text` file-offset
  space this collector uses everywhere else (symbols.tsv / `cro info`),
  confirmed in `AGENTS.md`'s own offset gotcha — no rebase needed.
- Wired through the live dashboard (`page.py`'s JS: STAGES ladder, legend,
  module bars/meta, SVG/PNG export), `summary.md`'s table, and the
  committed README assets — the `xy-viz` Rust crate (`charming`/ECharts)
  gained a 5th treemap/progress-bar tier (`#9b59b6`, priority between
  "hint" and "verified" in `TIERS`), refactored its progress-bar loop to
  index into `tier_counts()` positionally instead of re-deriving by bit
  (less error-prone when adding a tier). `assets.py`'s unused-but-
  documented-as-kept-in-sync `COLORS`/`LABELS`/`cls_of`/`counts_of` were
  updated too even though that renderer isn't currently invoked (`xy-viz`
  is what `readme` actually shells out to).
- Verified end-to-end: `heatmap.py readme` now reports "215 armcc
  masked-matched" matching the filesystem ground truth exactly
  (`find xy-decomp/armcc-matching/matched -name '*.c' -o -name '*.cpp' |
  wc -l`), and the new tier's color/label appear in both committed SVGs.

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

**Confirmed to recur in a second module (2026-10-15):** `DllNumberInput`'s
three destructor-stage functions (`0x1974`, `0x1a44`, `0x1a7c` — all the
"install dtor vtable, free an owned member if present, optionally
tail-call operator delete" shape) hit exactly this vptr-install
scheduling mismatch: armcc 5.04u5 always schedules the vtable-pointer
pool load before the member load and assigns it whichever register is
issued first (so the pool load wins `r0`), while the retail binary
schedules the member load first and puts the pool load in `r1` — the
*opposite* preference. This held regardless of C statement order,
intermediate-variable use, or expression fusion, strengthening the
version-skew theory: this isn't a source-shape problem, it's armcc
5.04u5 making a different latency-hiding choice than whatever built the
retail binary.

### 9 more functions matched in this repo (2026-10-15)

Working from this repo's own freshly extracted `DllDialogCommon.cro`
(no Ghidra available on this machine — no decompiled-C corpus to start
from), the next-easiest unclaimed bytes in `.text` turned out to be two
runs of trivial 1-2 word leaf functions sitting next to functions already
matched: `0x2d8`-`0x2ff` (six stubs, right before the `0x300` state
machine) and `0xd10`-`0xd1f` (a pair, between the `0xcdc` destructor and
the `0xd20` dispatcher). All 9 masked-match; see the updated table in
`xy-decomp/armcc-matching/README.md` for the per-offset breakdown and the
weak-evidence caveat on the four single-word `bx lr` stubs (any
no-op void function compiles to that same word, so the match proves
"at most this short," not "this body").

### Toolchain generalizes to a second module (2026-10-15)

`DllNumberInput.cro` was fingerprinted next, to check whether the
trivial-stub pattern and the fingerprinting approach were specific to
`DllDialogCommon` or general. They generalize: the same 1-2 word
virtual-override stubs recur (`0x1888`, `0x1a40`, `0x1ab0`), plus three
non-trivial functions — a field-copy-and-veneer-call helper (`0x18dc`,
8/8), a 19-word notification dispatcher (`0x1890`, 19/19), and a
30-word constructor (`0x18fc`, 30/30) whose node-initialization tail is
written as a **sequence of moving-pointer-cursor stores** (`str rX,
[rY,#N]!` pre-indexed writes that advance the pointer as a side effect)
rather than fixed-offset struct-member access — translating it the same
way in C (`p += N; *p = v; p += M; ...`) matched on the first try. See
`xy-decomp/armcc-matching/README.md` for the per-offset table.

`0x1890` surfaced a real decompilation lesson, not just a toolchain
check: it has two early-return paths that share **one epilogue**
(`pop {r4, pc}`) with no `mov r0, #N` before it. armcc returns whatever
value is already live in `r0` at each branch site rather than
re-materializing a return constant — the null-handle check falls through
`cmp r0, #0` with r0 still 0 (so that path returns 0), and the
already-notified check falls through `cmp r0, #1` with r0 still holding
the compared flag byte 1 (so that path returns 1). The semantically
"obvious" guess — both paths `return 1;` as boolean failure — compiled
to one extra instruction and didn't match; matching required tracing
the actual register value at each branch, not the value that reads as
true/false to a human. Generalizable technique: when two branches share
a physical epilogue with no constant load before it, check what's
already in the return register at each branch site before guessing the
return value.

### Open: `0xd20` block-layout puzzle (not yet solved)

`DllDialogCommon@0xd20` (size `0x8c`) is a message dispatcher: loads a
handler object from `this+8`, and for `msg` in `{4,10}`/`{5,13}`/`{16}`
calls `handler->vtbl[0](handler, 0|1|2)`, else returns 1 (0 on any call
taken). The control-flow graph was fully recovered by hand from the
disassembly and is not in doubt — reproducing it in C as both a
`switch` and as an equivalent `goto`-chain compiles to the **same wrong
block order**: armcc places the `msg>10` sub-check block (`0xd54`
originally) immediately after the initial comparison chain, with an
inserted skip-branch, and sinks the "return 1" default block afterward —
the opposite of the real binary's order, which falls straight through
into the default right after the initial checks and reaches the `>10`
sub-check only via the explicit `bgt`. 11/35 masked words matched before
this diverges (see `fingerprint_fn.py`'s per-word diff for the exact
shape). Not blocked on the toolchain — this is armcc's own block-layout
heuristic doing something source-order doesn't control directly. Needs
either a different C shape (nested-if variations not yet tried) or
accepting this specimen needs richer tooling than hand-written candidate
C. Tracked here rather than attempted further this session.

### Open: `0x710` near-match, same family of puzzle (not yet solved)

`DllDialogCommon@0x710` (size `0x94`) is a constructor: fetches an object
via an import veneer, installs a vtable pointer, calls two more veneers
to build a buffer (one of which takes the previous veneer's return as an
implicit second argument — not obvious from the call site alone), then
placement-allocates a second small object and zero-initializes several
of its fields using a pointer recovered by subtracting a fixed offset
(8) from an import veneer's return value. 30/37 masked words matched —
every value and the final state is correct — but the real binary
computes that `-8` adjustment **immediately** after the veneer call and
reuses the adjusted pointer with small positive offsets for every
subsequent field write, while armcc 5.04u5 at `-O2 -Otime` keeps the
*unadjusted* pointer live and folds the `-8` into each field's immediate
offset instead, only materializing the adjusted pointer at the very end
where it's stored as a return value. This reordering survived every
C-shape variant tried (typed struct member access vs. raw byte-offset
casts, splitting the subtraction into its own statement, forcing an
early use) and also survived `-O1`/`-O3`/`-Ospace` (all strictly worse
matches, not better).

**Update:** the sibling specimen `DllNumberInput@0x18fc` (same overall
shape: fetch object, install vtable, build buffer, placement-allocate a
node) was fingerprinted afterward and matched 30/30 on the first try —
but its node-init tail genuinely *is* written as a sequential
`pointer += delta; *p = v;` cursor chain in the retail binary (confirmed
by its own pre-indexed `str rX,[rY,#N]!` instructions), unlike `0x710`'s
tail, which really does use one `sub` followed by fixed offsets off a
stable base. So the two specimens use genuinely different source idioms
rather than one hidden rule explaining both; `0x710`'s specific
early-materialization gap remains unexplained and still open.

### Open: `0xdd8` jump-table specimen, plus a tool caveat (not yet solved)

`fingerprint_fn.py` gained `--mask-ranges offset:length[,...]` (byte
ranges relative to the function's own start) to exclude a jump table's
raw address words from comparison — those words are link-time facts,
zero in the unlinked CRO, so an isolated compile can never reproduce
them. **This proves only that the right number of case bodies exist, in
the right order, each byte-correct — it does not and cannot prove which
case value selects which body**, since that mapping is exactly what the
masked table encodes. Any case-number assignment in a candidate using
this flag is an inference from physical body order (assuming armcc
emits bodies in ascending case-value order), not a verified fact; the
tool prints this caveat whenever the flag is used, and a matched file
built this way must state the assignment as a hypothesis, never as
"case N does X."

First attempt with the flag, `DllDialogCommon@0xdd8` (a 5-case property
get/set switch, no function calls anywhere in it), did not reach a full
match, so no case-to-block mapping was ever asserted for it. Two
unresolved gaps: (1) the retail binary has a `push {r4, lr}` / `pop {r4,
pc}` frame that the candidate — which makes no calls — had no reason to
need; (2) the default case (selector ≥ 5) loads three pool constants in
a row with `e320f000` NOP-hint words interleaved between them in the
retail binary, which is this project's own marker for post-link
branch-relaxation nops (see `--relax-nops` above) — suggesting the real
default case made calls that were relaxed away entirely, not just
argument-setup loads with nothing to call. Both point at the same
missing piece (a call somewhere I haven't identified), which would
plausibly explain the frame too. Left open; `DllNumberInput@0x1af8` has
the same overall shape and wasn't attempted, since it would likely hit
the same gap.

### Dead end avoided (now corrected)

The earlier framing — "compiler parity (matching **GCC** version/flags)"
— was simply the wrong toolchain family. `xy-recomp`'s GCC-based builds
(devkitARM, static recompilation) remain the right tool for *that* track
(interpreting/recompiling to a portable C target doesn't need to
byte-match the original compiler); it's specifically **matching-decomp's
byte-identical goal** that needs armcc, not GCC. `docs/general/findings.md`
has been updated to point here instead of repeating the GCC assumption.
