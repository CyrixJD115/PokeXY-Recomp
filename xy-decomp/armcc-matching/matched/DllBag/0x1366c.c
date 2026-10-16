/* DllBag function @ CRO file offset 0x1366c, size 0x14 -- MATCHED (masked, 5/5).
 * calls fn(this->field_0x24), returns 1
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBag 0x1366c 0x14 xy-decomp/armcc-matching/matched/DllBag/0x1366c.c
 */
extern void fnA(void *x);
struct S { unsigned char pad[0x24]; void *p; };
int f(struct S *this) { fnA(this->p); return 1; }
