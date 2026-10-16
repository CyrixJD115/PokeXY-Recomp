/*
 * DllBattle @ 0x3342c, size 0x2c (incl. 1 vtable pool word) -- constructor stage: runs the
 * base ctor fnBase(o), then installs its own vtable at +0 and zeroes four int fields at
 * +56/+60/+64/+68; returns the object.
 * Masked match 11/11.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x3342c 0x2c xy-decomp/armcc-matching/matched/DllBattle/0x3342c.c
 */
extern void *fnBase(void *o);
extern char VT[];
struct O { void *vt; char pad[52]; int a; int b; int c; int d; };
struct O *fn(struct O *o) {
    struct O *p = (struct O *)fnBase(o);
    p->a = 0;
    p->vt = VT;
    p->b = 0;
    p->c = 0;
    p->d = 0;
    return p;
}
