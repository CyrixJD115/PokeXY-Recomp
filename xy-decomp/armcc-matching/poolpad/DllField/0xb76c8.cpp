/*
 * DllField @ 0xb76c8, size 0x5c -- secondary-base destructor: installs the sub-object vptrs (v, v+0x20 at +0x38,
 * v+0x30 at +0x50), releases the object at +0x5c via f1(o->p5c, o+0x50), installs the final vtable, runs the
 * inline Reset() (if state at +0x20 is set, f2(this, this+0x20)), then returns f3(o+8)-8. Sibling of DllField@0xb6844 (same body, different callee targets).
 * POOL-PAD match (21/23 words, 22 after dropping the retail alignment pad word): retail starts at 4 mod 8,
 * so one all-zero word precedes the (relocated, zero) pool constant and the ldr displacement is 4 larger.
 * Not a plain MASKED MATCH -- see README "Pool-pad tier".
 * Style: trick(declaration-order) -- `char *b = v + 0x30;` must be declared BEFORE `char *a = v + 0x20;`; the named locals themselves are also needed (inlined `v + 0x20`/`v + 0x30` stores and the a-before-b order both fail, checked 2026-10-16)
 *   to make armcc keep v+0x30 in r2 and v+0x20 in r0 (other orders: 16-19/23, head wrong).
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xb76c8 0x5c xy-decomp/armcc-matching/poolpad/DllField/0xb76c8.cpp --pool-pad
 */
extern void f1(void *, void *);
extern void f2(void *, void *);
extern void *f3(void *);
extern char vtbl[];
struct O {
    char *vt; char pad[0x34]; char *p38; char pad2[0x14]; char *p50; char pad3[8]; void *p5c;
    char *Pstate() { return *(char **)((char *)this + 0x20); }
    void Reset() { if (Pstate()) f2(this, (char *)this + 0x20); }
};
char *fn(O *o, char *v) {
    char *b = v + 0x30;
    char *a = v + 0x20;
    o->p38 = a;
    o->vt = v;
    o->p50 = b;
    f1(o->p5c, (char *)o + 0x50);
    o->vt = vtbl;
    o->Reset();
    return (char *)f3((char *)o + 8) - 8;
}
