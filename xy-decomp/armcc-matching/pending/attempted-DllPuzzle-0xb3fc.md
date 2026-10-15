## DllPuzzle @ 0xb3fc (and twin 0xbf80), size 0x38
- What it is: dtor stage: store VT, if (this->f14) { fn1f8(f14); f14 = 0; }, tail-call base fn268(this).
- Tried: plain C; local-variable `m` variant. Tail call (void* return) works fine.
- Stuck on: 9/14 words. Retail interleaves `ldr r1,pool; ldr r0,[r0,#20]; str r1,[r4]` and uses r1 for the zero (`mov r1,#0`); armcc emits pool-load into r0, str, then member load, mov r0,#0. Known-bad dtor shape (same residue as the member-load-ordering entry above). 0xbf80 differs only in using r0 for the zero.
