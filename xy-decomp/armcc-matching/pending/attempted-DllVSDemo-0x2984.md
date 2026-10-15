## DllVSDemo@0x2984 (size 0x20)

- What it is: prologue-less leaf, same idiom as the already-matched
  `DllDialogCommon@0x6e8` — loads an owned object pointer from
  `this+0x7c`; if null, returns 1; else tail-calls virtual slot 0 on it
  (`bx rX`) with the object itself as the new `this`.
- Tried: (1) nested `O`/`T` struct with `int (**vtbl)(O*)` field (same
  shape as the matched `0x6e8` reference), accessed as `o->vtbl[0](o)`.
  (2) same but with an explicit intermediate `vt` local variable holding
  `o->vtbl` before indexing. (3) fully untyped `void*` double-deref
  through casts, function pointer assigned to a separate local before
  calling. All three variants, across `-O1`/`-O2`/`-O3`/`-Ospace` x4
  flag combinations (12 compiles total).
- Stuck on: 5/8 masked words (best, variants 1 and 2 and -O2/-O3 tie).
  The null-check and the final `bx lr`/early-return path match exactly;
  the two-instruction vtable-dereference-then-tail-call sequence never
  reproduces. The retail binary picks **r1** as its scratch register for
  both the vtable-pointer load and the function-pointer load
  (`ldr r1,[r0]; ldr r1,[r1]; bx r1`); every C idiom tried instead makes
  armcc 5.04u5 pick **r3** for the same two loads + tail branch
  (`ldr r3,[r0]; ldr r3,[r3]; bx r3`) — otherwise byte-identical
  structure, just the wrong register number throughout. `-O1` destabilizes
  further (moves the early-return `mov r0,#1` after the branch instead of
  before, dropping to 3/8); `-Ospace` drops the `beq` in favor of a
  conditional `bxeq lr` (3/8, 7 words not 8).
- Lead for next time: unlike `0x6e8` (reached via a two-hop nested struct
  `t->p24->p4`), this specimen's object pointer is a single direct field
  (`t->obj` at `+0x7c`) — the shallower pointer chain may be exactly what
  changes armcc's register-pressure heuristic and picks r3 over r1. Not
  tried: deliberately keeping the struct two levels deep (an unnecessary
  wrapper struct) purely to match 0x6e8's register-pressure profile, or
  adding an unused dead local variable before the vtable dereference to
  perturb the allocator's free-register ordering.
