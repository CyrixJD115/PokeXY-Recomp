/*
 * DllBattle @ 0x39884, size 0x150 -- state classifier via a chain of virtual calls: returns 0 if vt[0x42](this) is set or
 * state=vt[0x10](this) is not 1..3; otherwise runs vt[0x1f](this,1), r=vt[0x20](this,1); r in {4,39..45} -> 3; else
 * probes vt[0x3a], vt[0x39], vt[0x35] on vt[0x19](this,0) -> 2, or 1 if the last returns -1.
 * Masked match 84/84.
 * Style: natural -- vtable declared as unprototyped fn ptrs because slots have different arities
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x39884 0x150 xy-decomp/armcc-matching/matched/DllBattle/0x39884.c
 */
typedef struct O { int (**vt)(); } O;
int fn(O *o) {
    int s, r;
    if (o->vt[66](o)) return 0;
    s = o->vt[16](o);
    if (s == 0) return 0;
    if (s == 1 || s == 2 || s == 3) {
        o->vt[31](o, 1);
        r = o->vt[32](o, 1);
        if (r == 4 || r == 39 || r == 41 || r == 42 || r == 44 || r == 45 || r == 40 || r == 43)
            return 3;
        if (o->vt[58](o, o->vt[25](o, 0))) return 2;
        if (o->vt[57](o, o->vt[25](o, 0))) return 2;
        if (o->vt[53](o, o->vt[25](o, 0)) == -1) return 1;
        return 2;
    }
    return 0;
}
