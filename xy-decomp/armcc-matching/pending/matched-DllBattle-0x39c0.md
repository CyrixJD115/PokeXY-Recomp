Module: DllBattle
Offset: 0x39c0
Size: 0x28
Match: 10/10
One-line: virtual-override: if (call(this)) this->+4->+0x14 = 1; return 1
Note: needs `B *b = self->b; int one = 1; b->f = one;` -- direct `b->f = 1` gives swapped r0/r1 (7/10).
