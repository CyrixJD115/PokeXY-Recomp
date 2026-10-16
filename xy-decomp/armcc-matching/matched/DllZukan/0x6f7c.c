/* DllZukan function @ CRO file offset 0x6f7c, size 0x1c -- MATCHED (masked).
 * If member at +0xc8 is non-null, tail-call its vtable slot 1 (byte +4) forwarding one extra arg.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllZukan 0x6f7c 0x1c xy-decomp/armcc-matching/matched/DllZukan/0x6f7c.c
 */
typedef struct O { void **vt; } O;
typedef struct T { unsigned char pad[0xc8]; O *m; } T;
void F(T *this, int a)
{
    O *o = this->m;
    if (o) ((void (*)(O *, int))o->vt[1])(o, a);
}
