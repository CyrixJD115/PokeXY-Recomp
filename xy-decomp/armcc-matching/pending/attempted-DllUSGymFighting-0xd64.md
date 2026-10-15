## DllUSGymFighting@0xd64 (size 0x50)

- What it is: short-circuit AND of two virtual-call predicates. Sibling of
  the matched `0xdb4` array-indexed predicate, same vtbl-slot-55 (offset
  0xdc) idiom, non-indexed: `return this->f30->vtbl[55](this->f30, 0) &&
  this->f2c->vtbl[55](this->f2c, 0);` (`f2c`/`f30` = fields at offsets
  0x2c/0x30, i.e. two adjacent `Obj*` members 4 bytes apart).
- Tried: (1) both fields loaded into locals `a`/`b` before the branch,
  declared/assigned in `a,b` order. (2) same but `b,a` order. (3) same
  with an explicit `self = this;` alias before the field reads. (4)
  direct field access with no intermediate locals at all (`this->f30->...`
  inline). All four compiled to the retail's own instruction *content*
  for the calls and loads (confirmed by word-for-word identical bodies
  once the register-naming offset is accounted for), but every variant
  either used the wrong register to hold the first operand across the
  call (`r4` holding `this` and re-dereferencing after the branch, not
  holding `b` directly pre-computed) or skipped an extra `mov r1, r0`
  the retail binary performs right at function entry, before the `push`.
- Stuck on: best result 9/20 masked words (variant 1, `a,b` order) --
  the middle (first virtual call through the loaded `a`) and tail (second
  virtual call through `r4`, final bool normalization, `pop`) match
  exactly; only the function's first ~4 words differ. Retail's shape is:
  `mov r1, r0` (save `this` into r1) BEFORE `push {r4, lr}`, then after
  the push, `ldr r0,[r0,#48]` (a = this->f30, still using original r0)
  and `ldr r4,[r1,#44]` (b = this->f2c, via the *preserved* r1 copy) --
  i.e. the retail binary computes `a` through `r0` and `b` through a
  register-saved copy of the original `this`, rather than just loading
  both off the live `r0`/computed pointer directly (which is what every
  candidate variant did instead, 1 instruction shorter and arguably more
  optimal). No source phrasing tried made armcc choose to preserve a
  second live copy of `this` across the first field access instead of
  just reading both fields off the same base pointer before it's
  clobbered.
- Lead for next time: this smells like a *statement-count*/optimizer-
  barrier effect rather than a load-order effect -- maybe the real source
  has an intervening statement between reading `a` and `b` that this
  reconstruction is missing entirely (something that makes `this` appear
  "live across a barrier" from armcc's point of view, forcing the extra
  register-save). Not tried: `register` hints, `volatile` on `this`, or
  wrapping the two field reads in a tiny helper that takes `this` by
  value (forcing an AAPCS-style parameter copy). Time-boxed at ~20
  minutes without success; a fresh angle is needed, not more order
  permutations of the same three statements.
