## DllUSGymFighting@0x11dc (size 0x2c)

- What it is: no-arg globals-init function, right after the matched
  `0x11c0` struct zero-initializer. Sets a 64-bit module-static global to
  -1 (`ldr r2,[pc]@lit; mvn r0,#0; mov r1,r0; strd r0,[r2]`) then a second
  8-byte global (two ints) to 0 (`ldr r0,[pc]@lit; mov r1,#0; mov r2,r1;
  stm r0,{r1,r2}`), then `bx lr`. Two literal-pool words at 0x1200/0x1204
  hold the two globals' (unresolved, pre-relocation) addresses.
- Tried: (1) `extern long long g_sentinel; extern struct{int a,b;}
  g_zero; void f(void){ g_sentinel=-1; g_zero.a=0; g_zero.b=0; }`. (2)
  same but with the -1 value materialized through a named local
  `long long v=-1;` before the assignment, hoping to force value-first
  register allocation.
- Stuck on: 3/11 masked words (both variants identical). This is the
  exact same residue class already logged for `DllFieldDebug@0x180`
  (see attempted.md): armcc 5.04u5 always allocates the destination
  **address** into the lowest free register (r0) and the 64-bit value
  into the next pair (r1:r2, not eligible for `strd` since the start
  register is odd), forcing a generic `stm` -- while the retail binary
  computes the value first into the even-aligned r0:r1 pair and the
  address second into r2, enabling the specialized `strd`. Order of the
  two global-assignment statements in source did not change which one
  got the "address-first" vs "value-first" treatment; the compiler always
  put whichever assignment it emitted first through the address-first
  path. The second global's store direction (stm merge of two int
  stores) is a secondary knock-on effect of the first assignment
  consuming r0 for its address.
- Lead for next time: `DllFieldDebug@0x180`'s own note flags an untried
  idea -- an un-inlined helper (true extern-linkage function, not a
  `static` one the compiler can inline) called as `set_u64(value,
  &dest)`, so the AAPCS calling convention forces value into r0:r1 and
  the pointer into r2 regardless of what the caller does around it. Not
  attempted here due to the per-function time budget; worth trying next
  time this residue class comes up, confirmed now present in at least
  two independent modules.
