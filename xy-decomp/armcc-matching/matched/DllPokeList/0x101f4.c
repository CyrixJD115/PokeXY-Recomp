/* DllPokeList function @ CRO file offset 0x101f4, size 0x2c -- MATCHED (masked, 11/11).
 * Style: natural
 * Getter: true when the field at +0x40 is zero and the field at +0x44 is
 * even (tst #1) or zero. Note armcc needs the `& 1` test written first so it
 * emits `tst; cmpne #0` rather than `cmp; tstne`.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllPokeList 0x101f4 0x2c xy-decomp/armcc-matching/matched/DllPokeList/0x101f4.c
 */

struct S { unsigned char pad[0x40]; unsigned a; unsigned b; };

int getter(struct S *this)
{
    if (this->a != 0) return 0;
    return (this->b & 1) == 0 || this->b == 0;
}
