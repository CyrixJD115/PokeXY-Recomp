# Attempted — not yet matched

Functions a decompiling agent spent real effort on (up to the 30-minute
per-agent time cap — see README.md's "Working on this" section) without
reaching a masked match. **Check this list before picking a candidate.**
If a function is already here, skip it unless you have a genuinely new
idea beyond what's listed — don't just retry the same C shape someone
else already tried.

This is a working log, not a polished doc: newest entries at the top,
terse is fine. Once something here actually gets matched, move its
entry out (delete it) and add it to `README.md`'s table instead — this
file is only for things still unsolved.

Format per entry:

```
## Unmined-module survey 2026-10-15 (DllPokeViewer + all remaining Dll{Test*,Debug*} modules)

- Every module without a matched/ dir is an 8192-byte stub. Surveyed first
  words at 0x180 for all 26: 21 (incl. DllPokeViewer, DllDebugPokeMake,
  DllDebugProcLoop, DllTestAriizumi/Hosaka/Ikeda/Ikuta/Ishiguro/Ito/Mori/NCL/
  Nakahiro/Obata/Ohno/Onoue/SeqDemoView/Sogabe/Tamada/Taya/Tomoya) contain
  ONLY the nnroControlObject_ 5-case jump-table dispatcher (known dead end).
- DllTestIchiraku, Kawada, Kosaka, Nagihashi, Suzuki additionally have a
  leading globals-init function @0x180 (size 0x2c: `ldr r2,[pc]; mvn r0,#0;
  mov r1,r0; strd r0,[r2]; ldr r0,[pc]; mov r1,#0; mov r2,r1; stm r0,{r1,r2};
  bx lr`) -- same residue as DllUSGymFighting@0x11dc / DllUSGymIce@0x1974.
  (Kosaka differs in one word: 0xe1c200f8, strd of r0:r1 to [r2,#8].)
- Tried on DllTestIchiraku@0x180 (--mask-ranges 0x24:4,0x28:4): two named zero
  locals assigned to g_b[0]/g_b[1] -> 4/11 (address-first, stm for the -1);
  struct copy from a zeroed local -> 3/11. No improvement over prior notes.
- Conclusion: no easy candidates remain in unmined modules; no matches produced.
  Next batch should target partially-mined modules instead.

## <Module>@<offset> (size <hex>)

- What it is: <one-line guess at what the function does>
- Tried: <approaches / C shapes attempted>
- Stuck on: <match ratio reached, and exactly which words/behavior differ>
- Lead for next time: <anything that might unblock it, or "none yet">
```

---

## DllConfig@0x31b8 (size 0x20)

- What it is: no-branch, no-call field-copy helper. `r1` points to a
  3-word source struct (f0,f4,f8); copies those into `r0`'s fields at
  +0x38,+0x3c,+0x40, and *also* re-copies f0/f4 into +0x44/+0x48 (so
  f0/f4 are each written twice, to two different destinations).
- Tried: (1) flat field-by-field assignment, every statement order —
  compiles to 5 separate ldr/str pairs, no merging (11 words vs
  target's 8). (2) `d->sub = *s;` via a 3-word nested sub-struct at
  +0x38 plus the two extra re-reads — reproduces the target's `ldm
  r1,{r2,r3,ip}` read side exactly, but the write side merges into one
  `stmia r0!,{r2,r3,ip}` (full 3-word post-increment store), not the
  target's split. (3) same via an explicit local `S tmp = *s;` with
  three separate per-field stores in the target's own store order —
  identical over-merge, byte-for-byte.
- Stuck on: 5/8 masked words. The `ldm` line and the two +0x44/+0x48
  re-read/re-store lines already match. The 3 words in between never
  reproduce: armcc 5.04u5 at -O2 -Otime either doesn't merge adjacent
  stores at all, or merges all three into one block op — never the
  target's partial merge (one lone `str` + one `strd` pair, skipping
  the 3-word `stm`). Source statement order didn't change the outcome
  in either the flat or struct-copy case.
- Lead for next time: maybe the real destination isn't one flat struct
  — e.g. the far field (+0x40) could belong to a different member or a
  union arm that breaks armcc's "is this one aggregate" merge check
  while the +0x38/+0x3c pair stays recognized as adjacent. Not tried:
  `-O1`/`-O3`/`-Ospace` flag variants, or reaching +0x40 through a
  genuinely unrelated pointer variable rather than the same `d` base.

## DllDialogCommon@0xd20 (size 0x8c)

- What it is: message dispatcher — loads a handler from `this+8`, and
  for `msg` in `{4,10}`/`{5,13}`/`{16}` calls `handler->vtbl[0](handler,
  0|1|2)`, else returns 1.
- Tried: plain `switch`, equivalent `goto`-chain matching the exact
  observed branch order.
- Stuck on: 11/35 masked words. Control flow is fully correct; armcc's
  block-layout for the comparison tree places blocks in a different
  physical order than the retail binary no matter how the switch/goto
  is phrased.
- Lead for next time: none yet — this is armcc's own layout heuristic,
  not something source-order seems to control. Full detail:
  `docs/decomp/armcc-toolchain.md`.

## DllDialogCommon@0x710 (size 0x94)

- What it is: constructor — fetches an object via an import veneer,
  installs a vtable, builds a buffer via two more veneer calls,
  placement-allocates a second small object and zero-initializes
  several of its fields using a pointer recovered by subtracting a
  fixed offset (8) from a veneer's return value.
- Tried: typed struct member access, raw byte-offset casts, splitting
  the subtraction into its own statement, forcing early use of the
  adjusted pointer, `-O1`/`-O3`/`-Ospace` flag variants.
- Stuck on: 30/37 masked words — every value and the final state is
  correct, but armcc keeps the *unadjusted* pointer live and folds the
  `-8` into each field's immediate offset, only materializing the
  adjusted pointer at the very end, while the retail binary computes it
  immediately after the veneer call and uses small positive offsets
  throughout.
- Lead for next time: the sibling `DllNumberInput@0x18fc` (already
  matched) has the same overall shape but its node-init tail genuinely
  *is* a sequential pointer-cursor chain in the retail binary (confirmed
  by its own pre-indexed `str rX,[rY,#N]!`), so that idiom doesn't
  transfer here — `0x710`'s tail really is one `sub` + fixed offsets.
  Full detail: `docs/decomp/armcc-toolchain.md`.

## DllDialogCommon@0xdd8 (size 0xb0)

- What it is: a 5-case property get/set switch on `this`, no function
  calls anywhere in the reconstructed body.
- Tried: `switch` over the 5 case values with `--mask-ranges` to exclude
  the jump table's data words.
- Stuck on: never reached a full match — the candidate didn't need a
  stack frame (no calls), but the retail binary has `push {r4,lr}` /
  `pop {r4,pc}` anyway, which shifts every subsequent word. The default
  case (selector >= 5) also loads three pool constants in a row with
  `e320f000` NOP-hint words interleaved — this project's own marker for
  post-link branch-relaxation nops — suggesting the real default case
  made calls that got relaxed away entirely, not just argument-setup
  with nothing to call.
- Lead for next time: find what the default case actually calls; that
  would likely also explain the frame. `DllNumberInput@0x1af8` has the
  same overall shape and hasn't been attempted — expect the same issue.
  Full detail: `docs/decomp/armcc-toolchain.md`.
## DllFieldDebug@0x180 (size 0x2c)

- What it is: module's very first function (no import-veneer table at all
  in this tiny module — 0 imports, `.text` is only 0xdc bytes total). A
  no-call, no-branch leaf: sets one 64-bit module-static global (call it
  `g_handle`) to -1 (invalid-handle sentinel) and a second, independent
  64-bit global (`g_state`) to 0. Confirmed via the CRO's own internal
  relocation table: two entries patch this function's two literal-pool
  words (file offsets 0x1a4/0x1a8) to `.data_segment_base + 0` and
  `.data_segment_base + 8` respectively — two distinct 8-byte objects,
  8 bytes apart, inside the module's 0x14-byte `.data` segment.
- Tried: (1) two plain `extern long long` globals, `g_handle = -1;
  g_state = 0;` (both statement orders) — word count matches (11) but
  only the *second* store matches byte-for-byte (confirms `g_state = 0;`
  is the right idiom for that half). (2) same but `static` instead of
  `extern` — armcc notices the two statics are provably adjacent within
  the TU and fuses both stores into one `ldr`+`stmia r0!,...`+`stm`
  sequence (9 words, not 11) — wrong shape entirely, extern is required
  to block that fusion. (3) explicit `Pair{unsigned int lo,hi;}` struct
  with individual `.lo`/`.hi` field assignments for both globals — loses
  the `stm` merge entirely (falls back to separate `str`/`str` pairs,
  worse). (4) local `Pair` temp copy-assigned into `g_handle` — store2
  (`g_state`) matches exactly this way too (7/11 overall) but store1 is
  unaffected. (5) an inlined `static void set_u64(long long value,
  long long *dest) { *dest = value; }` helper called as
  `set_u64(-1, &g_handle);`, hoping the AAPCS value-then-pointer argument
  order would survive inlining into the right register roles — armcc
  fully erases the call and allocates identically to the plain-assignment
  case (no change).
- Stuck on: 7/11 masked words (best result, variant 4's second half
  matches; store1 never reaches more than partial credit). The *only*
  piece that never reproduces is store1's register role assignment:
  retail computes the **value** first (`mvn r0,#0; mov r1,r0`, landing in
  the r0:r1 even-aligned pair) and the **destination address** *second*
  (`ldr r2,[pc,...]`), then uses `strd r0,[r2]` (one store-pair
  instruction). Every C idiom tried instead makes armcc compute the
  **address first** (always lands in r0, the lowest free register) and
  the value second (`r1,r2` — odd-start, so `strd` isn't even available,
  forcing a `stm` instead). Store2 (`g_state = 0;`) *does* naturally
  reproduce retail's address-first/`stm` pattern, so that half is solved
  and not worth re-attempting.
- Lead for next time: the value-first/address-second/`strd` ordering
  looks like a *calling-convention echo* — i.e. the real source likely
  passes the 64-bit sentinel value through an inlined helper whose
  parameter order is `(value, dest_ptr)` (AAPCS would naturally put a
  64-bit first arg in r0:r1 and a pointer second arg in r2), the
  opposite of the `(dest_ptr, value)`/`memset`-shaped order that *does*
  match for `g_state`. Attempt (5) tried exactly this shape but the
  inliner discarded the register-role hint — worth trying with the
  helper left **un-inlined** (give it external linkage / `__attribute__
  ((noinline))` equivalent so armcc must honor the AAPCS call, then see
  whether a *second* invocation of the function, compiled as a true call
  with masked `bl`, reproduces the body) — not yet tried due to the time
  cap. Also untried: `-O1`/`-O3`/`-Ospace` flag variants, and whether
  declaring `g_handle` with an explicit `__align(8)` changes anything.
  The module's only other function (`0x1ac`, size ~0xa0, the
  `nnroControlObject_` export) is a `cmp r1,#5; ldrcc pc,[pc,r1,lsl#2]`
  5-case jump-table dispatcher — same hard shape already logged for
  `DllDialogCommon@0xdd8`/`DllNumberInput@0x1af8`/`DllCommWait@0xce0`,
  not attempted here.


## DllNuts@0x708 (size 0x20) and DllNuts@0x728 (size 0x28)

- What it is: the module's first two real functions, right after the
  import-veneer table. Both copy an 8-single-float block, plus a
  trailing int/pair field, from one struct to another.
- Tried: struct assignment, a nested float-only sub-struct, `memcpy(32)`,
  an element-wise loop, a manually unrolled per-element float
  assignment, and `--fpu=vfpv2`/`--fpu=softvfp+vfpv2` flag overrides.
- Stuck on: every variant lowers either to GPR `ldm`/`stm` (which needs
  `push`/`pop r4-r8`; the target doesn't push those) or to individual
  per-element `vldr`/`vstr` pairs (no `vldm`/`vstm` formation) — never
  the target's single merged 8-register VFP multiple-transfer
  (`vldmia`/`vstmia`).
- Lead for next time: this looks like a new residue class, distinct
  from the already-documented vptr-scheduling one — armcc 5.04u5
  doesn't appear to peephole-merge consecutive single-float scalar
  loads/stores into one `vldm`/`vstm`, and its generic aggregate-copy
  lowering (struct assignment / inlined `memcpy`) always picks GPRs
  regardless of the aggregate's element type. Worth a fresh angle (a
  `union`, an explicit vector intrinsic, or a differently-shaped source
  idiom entirely) — not logged as fully dead, just not solved in this
  pass.

## DllPokeViewer — whole module is a dead end (not a single-function attempt)

- What it is: this module's entire real `.text` is exactly one function,
  at `0x180` — the `nnroControlObject_` module-control dispatcher (a
  5-case jump-table `switch` with a `push {r4,lr}`/`pop {r4,pc}` frame
  and a default-case tail of pool-constant loads interleaved with
  `e320f000`/`mov r0,r0` NOP-hint words).
- Stuck on: this is the exact unresolved jump-table/NOP-hint residue
  class already logged above for `DllDialogCommon@0xdd8` and
  `DllCommWait@0xce0`, and separately for `DllFieldDebug`'s own
  `nnroControlObject_` (same shape). No prior agent has cracked this
  class.
- Lead for next time: the module offers no other candidate function —
  it's the only real code in the file — so there is nothing else to try
  here until the jump-table/NOP-hint class itself is solved. Skip this
  module entirely (don't re-investigate) until that happens.

## DllTradelist@0xb6d0 (size 0x14) and DllTradelist@0xb6e4 (size 0x14)

- What it is: two adjacent struct-copy helper functions, no
  branches/calls. `0xb6d0`: `ldr r2,[r1]; str r2,[r0]; ldrh r3,[r1,#4];
  strh r3,[r0,#4]; bx lr` (copies a 4-byte field then a 2-byte field).
  `0xb6e4`: `ldr r2,[r1]; ldr r3,[r1,#4]; str r2,[r0]; str r3,[r0,#4]; bx
  lr` (loads both fields, then stores both — two 4-byte fields).
- Tried: field-by-field assignment, reversed field order, `void*`
  params with local casts, unsigned vs signed short, whole-struct
  assignment (`*dst = *src;` — over-merges into `ldm`/`stm`), and a
  volatile-source variant that got the load-both-then-store-both shape
  right for `0xb6e4` but still picked the wrong registers/order.
- Stuck on: armcc 5.04u5 always reuses `r1` (the `src` pointer register,
  dead after its last load) for the second temp instead of allocating
  `r3`, and/or orders the second store before the first. Every C idiom
  tried either reproduces the retail register choice or the retail
  store order, never both at once. Best ratio reached: 3/5 (`0xb6d0`),
  2/5 (`0xb6e4`).
- Lead for next time: this is the same general residue class already
  documented for `DllConfig@0x31b8` (struct-copy store-merge/
  register-allocation quirks) — a case of "two adjacent struct-copy
  helpers, same symptom" rather than a new independent lead. Try
  passing the struct by value (not pointer) to see if parameter-register
  pressure changes allocation, or an explicit `register` hint, or
  splitting into a helper that copies through an intermediate local
  array.

## DllTitle@0xd568 (size 0x28) and DllTitle@0xd54c (size 0x8)

Two stuck candidates in the same dense getter/setter/predicate cluster
(`0xd530`-`0xd8ac`) as this module's four matched functions.

**`0xd568` (size `0x28`) is a 4-int equality predicate**: loads
`this+16`/`+24`/`+28`/`+20` (in that exact order) into `r3`/`r1`/`r2`/`r0`,
then XORs the `+16`/`+24` pair and the `+20`/`+28` pair, ORs the two
diffs, and returns 1 if zero else 0 — a classic `(a==b) && (c==d)`
compiled branchless via xor/or rather than cmp/cmp/and.

- Tried, in order: `(p[4]==p[6]) && (p[5]==p[7])` — wrong shape entirely,
  armcc kept it as short-circuit branches (12/10 words). `(p[4]==p[6]) &
  (p[5]==p[7])` — still branchy, not the xor/or trick.
  `((p[6]^p[4])|(p[5]^p[7]))==0` direct on a flat `int *p` — gets the
  right op shape but armcc evaluates the two xor subexpressions one at a
  time (5/10). Same expression with each of the 4 loads hoisted into its
  own named int temp, in the exact declaration order matching the target
  — reproduces the load order and instruction shape byte-for-byte
  (10/10 words present, same opcodes in the same slots), **6/10 masked
  match**. The remaining 4-word mismatch is pure register allocation:
  target puts `+16` in `r3` and `+28` in `r2`; this candidate swaps them
  (same values, same operations, swapped physical registers). Permuting
  xor operand order and factoring the two xors into named intermediates
  neither fixes nor changes the allocation (the intermediate version is
  actually worse, 4/10, different load order entirely).
- Lead for next time: not yet tried — reordering the 4 declarations as
  b,a,d,c or a,b,d,c (same load-sequence constraint, different source
  variable identity per load) might flip which temp lands in `r2` vs
  `r3`.

**`0xd54c` (size `0x8`) is `ldrd r0,[r0,#16]; bx lr`** — a straight
64-bit field getter.

- Tried: `long long` return cast through `char*`, a struct with an
  8-byte-aligned `long long` field at the right offset, an 8-byte
  `{int,int}` struct-by-value return. Every phrasing compiles to either
  two separate `ldr`s (alignment not provable through the cast) or an
  indirect-return calling convention (`add`/`ldm`/`stm`, treating `r0`
  as a hidden return-pointer parameter instead of putting the pair
  directly in `r0:r1`) — never the single `ldrd` the retail binary uses
  for what should be the simplest possible getter shape.
- Stuck on: this looks like the same family as the already-documented
  vptr-scheduling/dtor residues — a toolchain-version default the
  fingerprint harness's flag set doesn't trigger from any C source tried
  so far. A sibling getter at `0xd560` (`ldrd r0,[r0,#24]; bx lr`, same
  size/shape) will have the identical problem and shouldn't be
  re-attempted until this one is solved.

## DllUSGymFighting@0x11dc (size 0x2c)

- What it is: no-arg globals-init function, right after the matched
  `0x11c0` struct zero-initializer. Sets a 64-bit module-static global
  to -1 (`ldr r2,[pc]@lit; mvn r0,#0; mov r1,r0; strd r0,[r2]`) then a
  second 8-byte global (two ints) to 0 (`ldr r0,[pc]@lit; mov r1,#0;
  mov r2,r1; stm r0,{r1,r2}`), then `bx lr`.
- Tried: an `extern long long g_sentinel; extern struct{int a,b;}
  g_zero;` pair with plain field assignment, and the same with the `-1`
  value materialized through a named local before the assignment,
  hoping to force value-first register allocation.
- Stuck on: 3/11 masked words (both variants identical). This is the
  exact same residue class already logged for `DllFieldDebug@0x180`:
  armcc 5.04u5 always allocates the destination **address** into the
  lowest free register (`r0`) and the 64-bit value into the next pair
  (`r1:r2`, not eligible for `strd` since the start register is odd),
  forcing a generic `stm` — while the retail binary computes the value
  first into the even-aligned `r0:r1` pair and the address second into
  `r2`, enabling the specialized `strd`. Source statement order didn't
  change which assignment got the address-first treatment; the
  compiler always routes whichever assignment it emits first through
  that path.
- Lead for next time: `DllFieldDebug@0x180`'s own note flags an
  untried idea — an un-inlined helper (true extern-linkage, not
  `static`) called as `set_u64(value, &dest)`, so the AAPCS calling
  convention forces the value into `r0:r1` and the pointer into `r2`
  regardless of what the caller does around it. Confirmed now present
  in at least two independent modules.

## DllUSGymFighting@0xd64 (size 0x50)

- What it is: short-circuit AND of two virtual-call predicates, sibling
  of the matched `0xdb4` array-indexed predicate, same vtbl-slot-55
  (offset `0xdc`) idiom but non-indexed: `return this->f30->vtbl[55]
  (this->f30, 0) && this->f2c->vtbl[55](this->f2c, 0);`.
- Tried: four variants (locals in `a,b` order; `b,a` order; an explicit
  `self = this;` alias; direct inline field access with no
  intermediate locals). All four reproduce the retail instruction
  content for the calls and loads word-for-word, but every variant
  either uses the wrong register to hold the first operand across the
  call, or skips an extra `mov r1, r0` the retail binary performs right
  at function entry, before the `push`.
- Stuck on: best result 9/20 masked words — the middle (first virtual
  call) and tail (second virtual call, bool normalization, `pop`)
  match exactly; only the first ~4 words differ. Retail's shape:
  `mov r1, r0` (save `this` into `r1`) **before** `push {r4, lr}`, then
  after the push, `ldr r0,[r0,#48]` (load the first field still via the
  original `r0`) and `ldr r4,[r1,#44]` (load the second field via the
  *preserved* `r1` copy of `this`) — i.e. retail computes the two
  fields through two different base registers (one live `this`, one
  saved copy), rather than reading both off the same live pointer
  before it's clobbered (which every candidate did instead — one
  instruction shorter and arguably more optimal).
- Lead for next time: smells like a statement-count/optimizer-barrier
  effect rather than a load-order effect — maybe the real source has
  an intervening statement between the two field reads that this
  reconstruction is missing entirely, making `this` appear "live
  across a barrier" to armcc. Not tried: `register` hints, `volatile`
  on `this`, or wrapping the two field reads in a helper that takes
  `this` by value (forcing an AAPCS-style parameter copy). A fresh
  angle is needed, not more order permutations of the same three
  statements.

## DllVSDemo@0x2984 (size 0x20)

- What it is: prologue-less leaf, same idiom as the already-matched
  `DllDialogCommon@0x6e8` — loads an owned object pointer from
  `this+0x7c`; if null, returns 1; else tail-calls virtual slot 0 on it
  (`bx rX`) with the object itself as the new `this`.
- Tried: a nested struct with an `int (**vtbl)(O*)` field (same shape as
  the matched `0x6e8` reference), accessed directly or through an
  explicit intermediate `vt` local holding the vtable pointer before
  indexing; a fully untyped `void*` double-deref through casts with the
  function pointer assigned to its own local before calling — three
  variants across `-O1`/`-O2`/`-O3`/`-Ospace` (12 compiles total).
- Stuck on: 5/8 masked words best (variants 1/2, `-O2`/`-O3` tie). The
  null-check and final `bx lr`/early-return path match exactly; the
  two-instruction vtable-dereference-then-tail-call sequence never
  reproduces. The retail binary picks **r1** as its scratch register for
  both the vtable-pointer load and the function-pointer load
  (`ldr r1,[r0]; ldr r1,[r1]; bx r1`); every C idiom tried instead makes
  armcc 5.04u5 pick **r3** for the same two loads + tail branch —
  otherwise byte-identical structure, just the wrong register number
  throughout. `-O1` destabilizes further (moves the early-return
  `mov r0,#1` after the branch, dropping to 3/8); `-Ospace` drops the
  `beq` for a conditional `bxeq lr` (3/8, 7 words not 8).
- Lead for next time: unlike `0x6e8` (reached via a two-hop nested
  struct `t->p24->p4`), this specimen's object pointer is a single
  direct field (`t->obj` at `+0x7c`) — the shallower pointer chain may
  be exactly what changes armcc's register-pressure heuristic and
  picks `r3` over `r1`. Not tried: deliberately keeping the struct two
  levels deep (an unnecessary wrapper struct) purely to match `0x6e8`'s
  register-pressure profile, or adding an unused dead local variable
  before the vtable dereference to perturb the allocator's
  free-register ordering.

## DllUSGymIce@0x1974 (size 0x2c)

- What it is: a no-argument "globals init" function. Loads the address
  of global A (an 8-byte value) into `r2`, writes -1 to both words via
  `mvn r0,#0; mov r1,r0; strd r0,[r2]`, then loads the address of
  global B (also 8 bytes) into `r0` and zeroes both its words via
  `mov r1,#0; mov r2,r1; stm r0,{r1,r2}`. Verified with
  `--mask-ranges 0x24:4,0x28:4` (the function's own two unresolved
  data-relocation literals).
- Tried: `static int g_a[2]; static int g_b[2];` in the same file —
  armcc recognizes the adjacent statics and constant-folds the second
  address to `addr(g_a)+8`, totally wrong shape (3/11).
  `extern volatile long long g_a; extern int g_b[2];` (both
  external/unresolved so the compiler can't fold their relative
  addresses) gets the **first half exactly right** (bit-for-bit
  `strd` match) but the second half diverges — the compile reuses a
  single register and emits `str`/`str` (recognizing it's storing the
  same zero value twice), while the retail binary deliberately
  computes two separate zero registers and issues one combined `stm`.
  **Best result: 9/11 words**, only the last two differ. Marking
  `g_b` `volatile` too made no difference; a chained assignment
  `g_b[1] = g_b[0] = 0;` made it worse (7/11, different shape
  entirely).
- Working theory for next time: `strd` vs `stm` for a same-value pair
  store appears to hinge on whether the two source registers end up as
  an even-starting pair (`r0`/`r1` → `strd` available) or an
  odd-starting pair (`r1`/`r2` → `strd` illegal, falls back to `stm`)
  — a register-pressure side effect of which register holds the
  destination address, not something controllable from the C source
  directly. Getting armcc to deliberately materialize a *second*,
  redundant register for the already-known-zero second word (instead
  of reusing the first) might require the two stores to look
  genuinely non-redundant to the optimizer (e.g. two different named
  locals assigned to the same literal, or different modeled field
  widths). Related to, but a distinct variant of, the
  `DllFieldDebug@0x180`/`DllUSGymFighting@0x11dc` `strd`-vs-register-
  role residue — this one got much closer (9/11 vs 3/11).

## DllField@0xae58 (size 0x10)

- What it is: no-branch, no-call leaf — zeroes two adjacent 4-byte int
  fields at `this+20` and `this+24`. Retail: `mov r2,#0; str r2,[r0,#20];
  str r2,[r0,#24]; bx lr`.
- Tried: (1) plain two-statement zero-assignment, no extra params — armcc
  allocates the zero constant into `r1` (lowest free scratch register),
  not `r2`. (2) same with an added-but-unused second parameter (`int
  unused`) hoping to push the scratch register up to `r2` — no change,
  still `r1`.
- Stuck on: 1/4 masked words (only the final `bx lr` matches). Retail
  picks `r2` for the zero constant, every candidate picks `r1`.
- Lead for next time: this looks like the same general register-role
  residue class already documented for `DllFieldDebug@0x180`/
  `DllUSGymFighting@0x11dc`/`DllUSGymIce@0x1974` (armcc picks the lowest
  free register for a materialized constant unless something external
  forces otherwise), but for a 32-bit value instead of 64-bit, and without
  an address-register competing for the pairing. Not tried: a genuinely
  used (not just unused) second parameter that itself occupies `r1`
  before the zero-store, or writing the two stores through an
  un-inlined helper taking `(dest_struct*, int)` so AAPCS parameter
  passing forces `r1` to already hold something else. A twin function at
  `0xe380`/`0xe3fc` elsewhere in the same module (identical shape, `+24`/
  `+28` and `+20`/`+24` field pairs) will have the same problem — don't
  re-attempt until this is solved.

## DllHologramMail@0x10da8 (size 0x24)

- What it is: same idiom as the matched `0xd424`/`0xd13c` (deref a pointer
  field at `this+36`+4, call an import with an argument + that object,
  store literal 11 into `this+40`), but `this` is **adjusted by -4** first
  (`sub r4, r0, #4`), and the called-with argument is the function's 2nd
  parameter (r1) instead of the 3rd. Retail bytes:
  ```
  push {r4, lr}
  sub  r4, r0, #4
  mov  r0, r1
  ldr  r1, [r4, #36]
  ldr  r1, [r1, #4]
  bl   <import 0xd10>
  mov  r0, #11
  strb r0, [r4, #40]
  pop  {r4, pc}
  ```
- Tried: `T *t = (T*)((char*)this_raw - 4); void *obj = *(void**)((char*)t->f36+4); import_d10(arg1, obj); t->f40=11;` (two statement orderings: obj-computed-as-its-own-statement-first, and obj-computed-inline-inside-the-call-expression). Both compile identically.
- Stuck on: 2/9 masked words. armcc 5.04u5 always **folds the -4 adjustment into the field offset** and reads the pointer field directly off the original (unmodified) r0 (`ldr r0,[r0,#32]` — offset 36-4), rather than going through the already-computed adjusted pointer in r4 (`ldr r1,[r4,#36]`) the way the retail binary does. It also swaps which register holds the outer argument during the computation (r2 instead of r0). This happens regardless of C statement order — it's the optimizer's own value-numbering choice (both expressions compute the same runtime address, so it picks whichever is cheaper for its internal scheduling), not something textual reordering influences.
- Lead for next time: this looks like the same general family as the
  already-documented vptr-scheduling/register-role residues (see
  `DllVSDemo@0x2984`, `DllUSGymFighting@0xd64` elsewhere in this file) but
  specifically about "this minus a small constant" never surviving as a
  materialized register when the compiler can re-derive the same address
  by adjusting an immediate offset instead. Not yet tried: making the `-4`
  adjustment opaque to value-numbering (e.g. route it through a
  `volatile`-qualified intermediate, or an un-inlined real function call
  that returns the adjusted pointer so the compiler can't algebraically
  unify it with the original `r0`). This same residue almost certainly
  also blocks two siblings in this module: `0x10dcc` (identical shape,
  different masked callee `0xd78` and uses the 3rd argument instead of the
  2nd — not independently attempted, assumed same problem on sight) and
  the first half of `0x10cf0` (size 0x34, a two-call compound function
  whose first half is this exact idiom at field offset +24 instead of
  +36 — also not independently attempted for the same reason).
