Module: DllTitle
Offset: 0xd534
Size: 0x18
Match: 6/6
One-line: Field-copy helper (byte -> this+4, signed halfword -> this+0), no calls.
Note: Toolchain gotcha worth folding into the README/toolchain doc: a
direct `(int)*(short*)(src+2)` cast compiles to a fused `ldrsh`+`str`
(2 instructions), which does NOT match the retail binary's separate
`ldrh`+`sxth`+`str` (3 instructions). Matching required reading the
halfword into an `unsigned short` temp in its own statement first, then
casting-and-storing in a second statement -- splitting the unsigned load
from the sign-extension forces armcc to keep them as separate
instructions instead of fusing into ldrsh. This looks like the same
family of "source phrasing matters, not just value" residue already
documented for DllBattlePartySelect@0x1684 (movne/moveq ordering) -- an
unsigned-load-then-cast vs. a direct signed-load cast produce different
instruction counts even though both are value-equivalent in C.
