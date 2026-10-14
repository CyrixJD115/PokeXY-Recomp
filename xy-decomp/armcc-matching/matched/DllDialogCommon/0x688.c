/* DllDialogCommon function @ CRO file offset 0x688, size 0x3c — MATCHED (masked, 15/15). Calls local fn @0x300 on member +0x24, virtual call slot 8, returns r==0 as if/return-0/return-1.
 *
 * Verify: python3 tools/fingerprint_fn.py DllDialogCommon 0x688 0x3c test/fingerprint/dialog_common_0x688.c
 */

/* 0x688: call local fn @0x300 on member +0x24, virtual call slot +0x20, return r==0. */
typedef struct O O;
struct O { void (**vtbl)(O *self); };
typedef struct T {
    void *p0;
    O *p4;                   /* +0x04 */
} T;
typedef struct W {
    unsigned char pad[0x24];
    T *t24;                  /* +0x24 */
} W;
extern int fn300(T *t);      /* local function @ 0x300 */

int F(W *w)
{
    T *t = w->t24;
    int r = fn300(t);
    O *o = t->p4;
    if (o)
        o->vtbl[8](o);
    if (r)
        return 0;
    return 1;
}
