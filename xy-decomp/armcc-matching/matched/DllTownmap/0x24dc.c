/* DllTownmap function @ CRO file offset 0x24dc, size 0x8.
 *
 * Shape: trivial int field setter. Stores the 2nd argument into the
 * object at +0xec, void return. Same idiom as DllFieldEventPlayer@0x624
 * / DllFieldEventTakinobori@0x11d4.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTownmap 0x24dc 0x8 xy-decomp/armcc-matching/matched/DllTownmap/0x24dc.c
 */

struct S {
    unsigned char pad[236];
    int field_ec; /* +0xec */
};

void func_24dc(struct S *a0, int a1)
{
    a0->field_ec = a1;
}
