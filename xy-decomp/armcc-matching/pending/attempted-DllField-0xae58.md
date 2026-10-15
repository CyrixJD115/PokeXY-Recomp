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
  picks `r2` for the zero constant; every candidate picks `r1`.
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
