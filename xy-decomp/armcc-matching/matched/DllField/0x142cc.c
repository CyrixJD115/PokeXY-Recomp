/* DllField function @ CRO file offset 0x142cc, size 0x1c -- MATCHED (masked, 7/7).
 * Sets flag byte this+0x25=1, sub-object(this+0x20)->state(+0x1c)=0, tail-calls veneer(this->+4, 0).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x142cc 0x1c xy-decomp/armcc-matching/matched/DllField/0x142cc.c
 */

extern void fn2f8(void *p, int x);
struct S { unsigned char pad[28]; unsigned char st; };
struct O { void *pad0; void *m; unsigned char pad1[24]; struct S *s; unsigned char pad2[1]; unsigned char f; };

void Fn(struct O *o)
{
    o->f = 1;
    o->s->st = 0;
    fn2f8(o->m, 0);
}
