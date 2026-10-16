/* DllZukan function @ CRO file offset 0x2500, size 0x10 -- MATCHED (masked).
 * Tail-calls vtable slot 9 (byte +0x24) of the member at +0x44 (no null check).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllZukan 0x2500 0x10 xy-decomp/armcc-matching/matched/DllZukan/0x2500.c
 */
typedef struct O { void **vt; } O;
typedef struct T { unsigned char pad[0x44]; O *m; } T;
void F(T *this)
{
    O *o = this->m;
    ((void (*)(O *))o->vt[9])(o);
}
