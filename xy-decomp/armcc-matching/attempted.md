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
