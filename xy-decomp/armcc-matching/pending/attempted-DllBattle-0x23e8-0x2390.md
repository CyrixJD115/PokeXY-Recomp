DllBattle 0x23e8 (size 0x34, simple dtor, vtable store + free member+0x14) and 0x23b0 (same + tail call): candidate loads vtable ptr then stores then loads member; retail does ldr r1,pool; ldr r0,[r0,#20]; str r1,[r4]. 8/13 words. Known-bad dtor shape.
DllBattle 0x2390 (size 0x1c, ctor wrapper: bl base; vtable store; str arg +0x28): 5/7, armcc picks r2 for the vtable temp where retail uses r1.
