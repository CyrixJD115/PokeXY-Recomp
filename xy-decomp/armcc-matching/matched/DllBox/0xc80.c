/* DllBox function @ CRO file offset 0xc80, size 0x1c -- MATCHED (masked, 7/7).
 * Conditional accessor, same idiom as DllEggDemo@0x568 / DllBoxSearch@0x5b8:
 * veneer call gates between a member load (+68) and a re-materialized
 * constant 2.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBox 0xc80 0x1c xy-decomp/armcc-matching/matched/DllBox/0xc80.c
 */

typedef struct {
    unsigned char pad[68];
    int field;
} Obj;

extern int Check(Obj *p);

int GetOrDefault(Obj *p)
{
    if (Check(p))
        return p->field;
    return 2;
}
