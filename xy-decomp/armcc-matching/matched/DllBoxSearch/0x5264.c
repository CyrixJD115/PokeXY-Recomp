/* DllBoxSearch function @ CRO file offset 0x5264, size 0x14 -- MATCHED (masked).
 * `if (v == 1) this->f88 = 2; return 0;`
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBoxSearch 0x5264 0x14 xy-decomp/armcc-matching/matched/DllBoxSearch/0x5264.c
 */
typedef struct O { unsigned char pad[136]; int f; } O;

int F(O *t, int v)
{
    if (v == 1)
        t->f = 2;
    return 0;
}
