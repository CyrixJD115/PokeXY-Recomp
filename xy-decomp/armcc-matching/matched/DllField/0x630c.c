/* DllField function @ CRO file offset 0x630c, size 0xc -- MATCHED (masked, 3/3).
 * tail-calls fn(this->[164], 0)
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x630c 0xc xy-decomp/armcc-matching/matched/DllField/0x630c.c
 */

extern void fn(void *, int);
struct S { char pad[164]; void *p; };
void Fn(struct S *s) { fn(s->p, 0); }
