/* DllField @ 0xb7bbc size 0x2c MATCHED (masked 11/11): secondary-base dtor stage (one of 24 identical sites).
 * `this->vptr = vt (arg r1); if (this->m@+32) Free(this, &this->m); return Base(this+8) - 8;`
 * Style: trick(volatile-store) -- the vptr store goes through a volatile pointer purely to pin the `mov r4,r0` copy
 *   ahead of the store; plain store sinks the copy below the ldr (8/11).
 * Mechanism (cracked 2026-10-16): armcc if-converts a call arm (`addne r1; blne`) only when the arm is <=2
 *   instructions. In C, the null-test load lands in r0 (r0 looks dead once `this` is copied to r4) so the arm
 *   needs `mov r0,r4` and armcc emits a `beq` branch instead. Written as an INLINE C++ MEMBER
 *   (`Reset() { if (m) fr(this, &m); }`) the test load goes to r1 and r0 stays `this`, giving retail's
 *   `ldr r1; cmp; addne r1,r0,#32; blne`. Same C body as a plain function: 4/11. Needs --cpp (.cpp suffix).
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xb7bbc 0x2c xy-decomp/armcc-matching/matched/DllField/0xb7bbc.cpp
 */
struct S;
extern void fr(S *self, void *p);
extern void *g(void *p);
struct S {
    void *vt;
    char pad[28];
    void *m;
    void Reset() { if (m) fr(this, &m); }
    S *Sub() { return (S *)((char *)this + 8); }
};
S *f(S *o, void *vt)
{
    *(void *volatile *)o = vt;
    o->Reset();
    return (S *)((char *)g(o->Sub()) - 8);
}
