Module: DllHologramMail
Offset: 0xe970
Size: 0x24
Match: 9/9
One-line: Calls an import veneer (0x190) with `this+8` and 0, then sets two adjacent byte flags (`this+17`, `this+16`) to literal 2.
Note: New trick worth folding in: retail keeps the post-call constant in r1 (not the dead r0) across both strb instructions. A flat `t->f17=2; t->f16=2;` reuses dead r0 instead (6/9). Wrapping the two stores in a tiny `static void set_flags(T*, unsigned char v){ t->f17=v; t->f16=v; }` helper called as `set_flags(t, 2)` pins the value into r1 via the AAPCS 2nd-parameter register at the (auto-inlined) call site — 9/9. Useful general lever for "dead call result, immediate constant written to multiple fields right after" shapes elsewhere.
