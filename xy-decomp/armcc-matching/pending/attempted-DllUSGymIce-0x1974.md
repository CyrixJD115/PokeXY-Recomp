Module: DllUSGymIce
Offset: 0x1974
Size: 0x2c (0x24 code + 8-byte own literal pool, two 4-byte words at
+0x24/+0x28 relative to function start, both all-zero in the raw CRO —
these look like unresolved data relocations, not link-time-folded
constants, since they're zero rather than a plausible compile-time-folded
offset)

What it is: a no-argument "globals init" function. Loads the address of
global A (an 8-byte value) into r2, writes -1 to both words via
`mvn r0,#0; mov r1,r0; strd r0,[r2]`, then loads the address of global B
(also 8 bytes) into r0 and zeroes both its words via
`mov r1,#0; mov r2,r1; stm r0,{r1,r2}`.

What was tried: `--mask-ranges 0x24:4,0x28:4` to exclude the two literal
addresses (a legitimate use per the tool's own docstring — these are
link-time facts, not case-table data, but the same "address an isolated
compile can't know" category).

- Declaring both globals as plain `static int g_a[2]; static int g_b[2];`
  in the same file: armcc 5.04u5 at -O2 recognizes they're adjacent
  statics in the same translation unit and constant-folds the second
  address to `addr(g_a)+8` (one literal load instead of two) — doesn't
  match the target's two-independent-literal shape at all (3/11).
- `extern volatile long long g_a; extern int g_b[2];` (treat A as a true
  64-bit object, B as two ints, both external/unresolved so the compiler
  can't fold their relative addresses): gets the **first half exactly
  right** (`ldr r2,[pc]; mvn r0,#0; mov r1,r0; strd r0,[r2]` — bit-for-bit
  match) but the second half diverges: my compile reuses a single `r1`
  register and emits `str r1,[r0]; str r1,[r0,#4]` (recognizing it's
  storing the same zero value twice, cheaper not to also materialize
  r2), while the retail binary deliberately computes BOTH `r1` and `r2`
  via `mov r1,#0; mov r2,r1` and issues one combined `stm r0,{r1,r2}` —
  best result so far: **9/11 words**, only the last two words differ.
- Marking `g_b` itself `volatile` too (to try to block the redundant-
  register-elimination optimization) made no difference — armcc produced
  the identical `str`/`str` pair regardless of `g_b`'s volatility.
- A chained assignment `g_b[1] = g_b[0] = 0;` made it worse (7/11) —
  different code shape entirely (back to a single reused register with
  `str`s at different offsets, and the two literal loads shift register
  assignment, breaking the already-matching first half too).

Working theory for next time: `strd` vs `stm` for a same-value pair store
appears to hinge on whether the two source registers end up as an
even-starting pair (r0/r1 → strd available) or an odd-starting pair
(r1/r2 → strd illegal, armcc falls back to stm) — a register-pressure
side effect of which register holds the destination address, not a
property controllable from the C source directly. The residue here is
getting armcc to deliberately materialize a *second*, redundant
register for the already-known-zero second word instead of reusing the
first — might require the two stores to look like genuinely
non-redundant values to the optimizer (e.g. two different named locals
assigned to the same literal, or restructuring so the real field widths
differ from what's modeled here). Not re-attempted further; flagged for
whoever next works this module or wants to fold the `strd`-vs-`stm`
observation into the toolchain residue-class notes.
