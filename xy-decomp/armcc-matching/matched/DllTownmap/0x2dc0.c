/* DllTownmap function @ CRO file offset 0x2dc0, size 0x14 — MATCHED (masked, 5/5).
 * Forwarder: tail-calls fn(this->[0x5c], 0, 0x104, x) (arg shuffle, no frame).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTownmap 0x2dc0 0x14 xy-decomp/armcc-matching/matched/DllTownmap/0x2dc0.c
 */
extern void fn(void *a, int b, int c, int d);
struct S { char pad[0x5c]; void *m; };
void f(struct S *s, int x)
{
    fn(s->m, 0, 0x104, x);
}
