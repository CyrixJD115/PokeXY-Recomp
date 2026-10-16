/* DllZukan function @ CRO file offset 0x1f14, size 0x1c -- MATCHED (masked).
 * If member at +0xe0 is non-null, tail-call its vtable slot 2 (byte +8) forwarding one extra arg.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllZukan 0x1f14 0x1c xy-decomp/armcc-matching/matched/DllZukan/0x1f14.c
 */
typedef struct O { void **vt; } O;
typedef struct T { unsigned char pad[0xe0]; O *m; } T;
void F(T *this, int a)
{
    O *o = this->m;
    if (o) ((void (*)(O *, int))o->vt[2])(o, a);
}
