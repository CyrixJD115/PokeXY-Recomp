/* DllField function @ CRO file offset 0x142e8, size 0x20 -- MATCHED (masked, 8/8).
 * Same as 0x142cc but sub-object state = 2.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x142e8 0x20 xy-decomp/armcc-matching/matched/DllField/0x142e8.c
 */

extern void fn2f8(void *p, int x);
struct S { unsigned char pad[28]; unsigned char st; };
struct O { void *pad0; void *m; unsigned char pad1[24]; struct S *s; unsigned char pad2[1]; unsigned char f; };

void Fn(struct O *o)
{
    o->f = 1;
    o->s->st = 2;
    fn2f8(o->m, 0);
}
