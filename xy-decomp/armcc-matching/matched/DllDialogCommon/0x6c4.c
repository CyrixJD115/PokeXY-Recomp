/* DllDialogCommon function @ CRO file offset 0x6c4, size 0x24 — MATCHED (masked, 9/9). Virtual call slot 2 on member +0x24, return 1. r4 push is AAPCS stack-alignment padding.
 *
 * Verify: python3 tools/fingerprint_fn.py DllDialogCommon 0x6c4 0x24 test/fingerprint/dialog_common_0x6c4.c
 */

typedef struct O O;
struct O { void (**vtbl)(O *self); };
typedef struct T { unsigned char pad[0x24]; O *p24; } T;

int F(T *t)
{
    O *o = t->p24;
    if (o)
        o->vtbl[2](o);
    return 1;
}
