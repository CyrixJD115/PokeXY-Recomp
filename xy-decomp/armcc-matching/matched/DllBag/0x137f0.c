/*
 * DllBag @ 0x137f0 (size 0x3c) -- masked match 15/15.
 * clears this+0x88; calls two helpers; if second returns nonzero and field still 0, sets it to 1 (void; r0 left live)
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBag 0x137f0 0x3c xy-decomp/armcc-matching/matched/DllBag/0x137f0.c
 */
extern void *fc68(void *);
extern int fc70(void *);
typedef struct { char pad[0x88]; int v; } O;
void fn(O *o){
    o->v = 0;
    fc68(o);
    if (fc70(o)) { if (o->v == 0) o->v = 1; }

}
