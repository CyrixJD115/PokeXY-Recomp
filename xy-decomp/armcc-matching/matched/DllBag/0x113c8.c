/* DllBag function @ CRO file offset 0x113c8, size 0x30 -- MATCHED (masked, 12/12).
 * dtor stage: o->vt = &vtbl (pool word); g1(o->sub->m); return (char*)g2(o+0x50)-0x50
 * Style: natural -- plain this-adjusting dtor stage ((char*)fn(o+0x50)-0x50 is the secondary-base adjust)
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBag 0x113c8 0x30 xy-decomp/armcc-matching/matched/DllBag/0x113c8.c
 */
extern void g1(void *p);
extern void *g2(void *p);
extern char vtbl[];
struct O { void *vt; void *pad; struct { unsigned char pad[0x30]; void *m; } *sub; unsigned char pad2[0x50 - 0xc]; };
void *f(struct O *o)
{
    o->vt = vtbl;
    g1(o->sub->m);
    return (char *)g2((char *)o + 0x50) - 0x50;
}
