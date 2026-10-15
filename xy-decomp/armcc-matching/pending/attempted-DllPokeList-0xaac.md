## DllPokeList@0xaac (size 0x54; same shape at 0xa50 size 0x5c)

- Tried (2026-10-15, new): the attempted.md lead. Three separate externs VT0/VT1/VT2 -> 8/21, WORSE (armcc emits 3 independent pool loads, no shared base). Single extern VT with (VT+16)/(VT+32) + --relax-nops -> 12/20 again (armcc loads pool after push, then sub/add chain off r0 instead of independent adds from r1).
- Lead disproven: separate externs do not force the retail shape. Retail loads pool into r1 before push; likely a different source construct (e.g. member-init in an inlined base dtor) - untried.
- 0x910 (size ~0x134) is a constructor with inlined sub-object ctors, stack-passed arg structs and virtual calls through [vt+8]; judged too large for this budget, not attempted past disassembly.
