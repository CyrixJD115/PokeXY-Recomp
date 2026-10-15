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
