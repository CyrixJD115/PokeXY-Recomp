/* DllBag function @ CRO file offset 0x11398, size 0x2c -- MATCHED (masked, 11/11).
 * dtor stage: o->vt = passed vt; g1(o->sub->m); return g3((char*)g2(o+0x50)-0x50) (tail call)
 * Style: natural -- plain this-adjusting dtor stage ((char*)fn(o+0x50)-0x50 is the secondary-base adjust)
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBag 0x11398 0x2c xy-decomp/armcc-matching/matched/DllBag/0x11398.c
 */
extern void g1(void *p);
extern void *g2(void *p);
extern void *g3(void *p);
struct O { void *vt; void *pad; struct { unsigned char pad[0x30]; void *m; } *sub; unsigned char pad2[0x50 - 0xc]; };
void *f(struct O *o, void *vt)
{
    o->vt = vt;
    g1(o->sub->m);
    return g3((char *)g2((char *)o + 0x50) - 0x50);
}
