Module: DllGift
Offset: 0xae0
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: First function claimed in a 43rd-ish module (DllGift). Sits right after an
unusually large (~696-entry) import-veneer table (0x180-0xadf). Six bare `bx lr`
single-word stubs follow (0xae8-0xaf8, weak-evidence class, unclaimed), then the
module's first real function at 0xafc (destructor-stage helper: vtable install,
free member @+0x14 if non-null, tail-call via `b 0x188` — the already-documented
dtor idiom, not attempted here), and a second similar dtor at 0xb34 (pop
{r4,pc} directly instead of tail-calling — load order not checked, flagged as
a lead for a future agent).
