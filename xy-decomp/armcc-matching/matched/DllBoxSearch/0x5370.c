/* DllBoxSearch function @ CRO file offset 0x5370, size 0x18 -- MATCHED (masked).
 * Secondary-base thunk twin of 0x5264: `this` is the sub-object at +132, so the
 * body subtracts 132 before storing at +136 (net store at sub+4).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBoxSearch 0x5370 0x18 xy-decomp/armcc-matching/matched/DllBoxSearch/0x5370.c
 */
typedef struct O { unsigned char pad[136]; int f; } O;

int F(char *s, int v)
{
    O *t = (O *)(s - 132);
    if (v == 1)
        t->f = 2;
    return 0;
}
