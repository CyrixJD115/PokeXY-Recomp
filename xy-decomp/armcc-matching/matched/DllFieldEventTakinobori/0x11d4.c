/*
 * DllFieldEventTakinobori @ 0x11d4, size 0x8 -- masked match 2/2.
 *
 * Trivial int field setter: stores the second argument into the object at
 * offset +0x18, void return. Sits right after a two-word literal pool
 * (0x11cc/0x11d0, feeding the preceding dispatcher's two `ldr pc,[pc,#N]`
 * loads) and right before the twin `return 1` stub at 0x11dc.
 *
 * Style: natural -- trivial stub/setter (weak evidence: any function of this shape compiles identically)
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllFieldEventTakinobori 0x11d4 0x8 \
 *       xy-decomp/armcc-matching/matched/DllFieldEventTakinobori/0x11d4.c
 */

typedef struct {
    unsigned char pad[24];
    int field_0x18;
} Obj;

void Obj_SetField0x18(Obj *this, int v) {
    this->field_0x18 = v;
}
