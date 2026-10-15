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

