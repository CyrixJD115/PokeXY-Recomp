Module: DllDebugPokeMake (identical 0xac function in DllDebugProcLoop, DllPokeViewer, ~17 DllTest* modules; also at 0x1ac in the 0xdc-sized ones)
Offset: 0x180
Size: 0xac

What it is: 5-way switch on r1 (`cmp r1,#5; ldrcc pc,[pc,r1,lsl #2]`, inline jump table of relocs) with push {r4,lr}. Cases in body order: *p=&X1; g=p (r2=&g); *p=g; p[0]=&X2,p[1]=&X3; p[0]=&X4,p[1]=&X5. Default (op>=5) returns 0xd8e12c21 through a weird tail: ldr r0,=0x30000000; nop; ldr r0,=0x30000026; nop,nop(0xe320f000 in the middle); ldr r0,=0xd8e12c21; pop {r4,pc} (dead loads of the first two constants survive).

Tried: plain switch C with extern int objects and (int)&Xn stores, default return 0xd8e12c21. Got 0 full match: armcc emits no push {r4,lr} (bx lr leaf, no shared tail), puts the default as a separate leaf, and the table layout differs. Case-body shape (ldr const / str / b shared tail) matches in spirit.
Lead: the dead 0x30000000/0x30000026 loads + nops look like inlined Result-construct helpers (nn::Result module/description bits) emitted unoptimized, and the r4 push suggests a call-free non-leaf frame; likely needs the original inline helper structure. Not worth more until a second example of that Result idiom is matched elsewhere.
