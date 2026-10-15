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
  `DllVSDemo@0x2984`, `DllUSGymFighting@0xd64` in `attempted.md`) but
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
