/* DllBag function @ CRO file offset 0xc558, size 0x10 -- MATCHED (masked, 4/4).
 * calls veneer(this), returns 42
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBag 0xc558 0x10 xy-decomp/armcc-matching/matched/DllBag/0xc558.c
 */
extern void fnA(void *t);
int f(void *this) { fnA(this); return 42; }
