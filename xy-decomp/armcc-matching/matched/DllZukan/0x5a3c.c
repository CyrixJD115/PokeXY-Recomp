/* DllZukan function @ CRO file offset 0x5a3c, size 0x10 -- MATCHED (masked).
 * Tail-calls vtable slot 9 (byte +0x24) of the member at +0x3c (no null check).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllZukan 0x5a3c 0x10 xy-decomp/armcc-matching/matched/DllZukan/0x5a3c.c
 */
typedef struct O { void **vt; } O;
typedef struct T { unsigned char pad[0x3c]; O *m; } T;
void F(T *this)
{
    O *o = this->m;
    ((void (*)(O *))o->vt[9])(o);
}
