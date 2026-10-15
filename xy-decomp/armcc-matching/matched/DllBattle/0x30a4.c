/*
 * DllBattle @ 0x30a4, size 0x68 -- destructor: install vtable, release owned obj at +0xc
 * (vcall slot 4, then slot 1 if still set, zero), free +0x38, call sub-object dtor at +0x18,
 * return result-0x18. RELAXED match 26/26 (one relaxation nop elided).
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x30a4 0x68 xy-decomp/armcc-matching/matched/DllBattle/0x30a4.c --relax-nops
 */
typedef struct O { void **vt; } O;
extern void *vtbl[];
extern void fa(void *);
extern char *fb(char *);
char *fn(char *self) {
    O *o;
    *(void ***)self = vtbl;
    o = *(O **)(self + 0xc);
    if (o != 0) {
        ((void (*)(O *))o->vt[4])(o);
        o = *(O **)(self + 0xc);
        if (o != 0) ((void (*)(O *))o->vt[1])(o);
        *(int *)(self + 0xc) = 0;
    }
    fa(*(void **)(self + 0x38));
    return fb(self + 0x18) - 0x18;
}
