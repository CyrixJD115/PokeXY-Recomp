/*
 * DllBattle @ 0x12fcc, size 0x40 -- destructor stage: install vtable, release member at +8 via vcall slot 1 if non-null, free member at +0xc if non-null (no zeroing), return this.
 * Style: trick(ordering-local) -- local `vt = VT` and hoisted member load only fix pool-load vs member-load order
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x12fcc 0x40 xy-decomp/armcc-matching/matched/DllBattle/0x12fcc.c
 */
extern char VT[];
extern void fn_free(void *p);
typedef struct O { void **vt; } O;
typedef struct D { void *vt; int pad; O *a; void *b; } D;
D *fn(D *this) {
    void *vt = VT;
    O *a = this->a;
    this->vt = vt;
    if (a) ((void (*)(O *))a->vt[1])(a);
    if (this->b) fn_free(this->b);
    return this;
}
