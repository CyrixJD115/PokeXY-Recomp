/* DllBag function @ CRO file offset 0x10c8, size 0x1c -- MATCHED (masked, 7/7).
 * Conditional accessor: if veneer(this) != 0 return this->field_0x3c else 2.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBag 0x10c8 0x1c xy-decomp/armcc-matching/matched/DllBag/0x10c8.c
 */
extern int fnCheck(void *p);

struct S { unsigned char pad[0x3c]; int v; };

int get_or_default(struct S *this)
{
    if (fnCheck(this)) return this->v;
    return 2;
}
