DllBox 0x1dfac (size 0x6c) and sibling 0x1df3c (size 0x70, same body + tail-call of a base dtor instead of `return o`)
Shape: vptr=VT1; if (o->[36]) f258(f910(f910(sub+108)-24)-84); f260(o->[32]); vptr=VT2; if (o->[20]) { f620(o->[20]); o->[20]=0; } return o.
Best: 0x1dfac 25/27 (c.c variant: `s=o->sub; o->vptr=VT1; ...; { void *p=o->buf14; if(p){f620(p); o->buf14=0;} } return o;`).
Only diff: retail zeroes via `mov r1,#0; str r1,[r4,#20]`, ours emits `mov r0,#0; str r0,[r4,#20]`.
Tried (all still r0): f620 returning void*, int-cast zero, `p=0; o->buf14=p`, reading p before the VT2 store (worse, 22/27), vt2 held in a local (worse, 9/27).
0x1df3c: same 20/28 with a mov r4,r0 ordering diff (retail loads VT and sub before `mov r4,r0`), plus the same r1-zero diff.
Lead: something keeps r0 live (or r1 free) across the zeroing store; try different inlined-helper shapes for the member release.
