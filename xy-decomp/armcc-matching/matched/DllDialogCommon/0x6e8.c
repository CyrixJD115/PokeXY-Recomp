/* DllDialogCommon function @ CRO file offset 0x6e8, size 0x20 — MATCHED (masked, 8/8). Prologue-less leaf missed by the catalog: tail virtual call (bx r1) slot 9.
 *
 * Verify: python3 tools/fingerprint_fn.py DllDialogCommon 0x6e8 0x20 test/fingerprint/dialog_common_0x6e8.c
 */

typedef struct O O;
struct O { int (**vtbl)(O *self); };
typedef struct U { void *p0; O *p4; } U;
typedef struct T { unsigned char pad[0x24]; U *p24; } T;

int G(T *t)
{
    O *o = t->p24->p4;
    if (o)
        return o->vtbl[9](o);
    return 0;
}
