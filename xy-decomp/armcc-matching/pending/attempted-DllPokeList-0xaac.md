## DllPokeList@0xaac (size 0x54, same shape at 0xa50 size 0x5c) -- multiple-inheritance dtor, 12/20 best

Shape: vtable pool word loaded into r1 BEFORE `push {r4,lr}`; then
r0=r1+16 -> str [this,#12]; r2=r1+32; str r1,[this]; str r2,[this,#32];
if (this[10]) vcall slot 1; r0=fnA(this+32); r0=fnB(r0-20); return r0-12
(pop {r4,pc}; has one relaxation nop before the second bl).

Tried (all with --relax-nops, struct-typed fields, locals `vt = VT`, char*/void**/unsigned*
vtable types, member-load before/after the stores): best 12/20. armcc always loads
the pool word AFTER `push/mov r4,r0` and reuses one register for vt, vt+16, vt+32
(chain add r1=r0+16 ... add r0,r0,#32) instead of keeping vt live in r1 with two
independent adds (r0=r1+16, r2=r1+32). Store order target: [12], [0], [32].
Lead: three separate vtable-pointer externs (VT0/VT1/VT2 at +0/+16/+32 of one
object) might force independent address materialisation; untried.
