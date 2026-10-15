/*
 * DllField @ 0x55384 (size 0x38) -- masked match 14/14.
 *
 * Same two-call-helper idiom as DllField@0x54e08: CallA(this,
 * this->f10, 0, 4, 8) then CallB(this->f0, 33); always returns 1. Same
 * field-type template, different literal constants -- confirms this is
 * a per-field-id instance of a shared helper shape rather than a
 * one-off.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField \
 *     0x55384 0x38 xy-decomp/armcc-matching/matched/DllField/0x55384.c
 */

typedef struct S {
    void *f0;
    unsigned char pad[6];
    unsigned char f10;
} S;

extern long CallA(S *o, long b, long c, long d, long e);
extern long CallB(void *p, long n);

long Func(S *o) {
    CallA(o, o->f10, 0, 4, 8);
    CallB(o->f0, 33);
    return 1;
}
