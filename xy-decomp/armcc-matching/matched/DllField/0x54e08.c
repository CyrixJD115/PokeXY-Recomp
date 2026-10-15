/*
 * DllField @ 0x54e08 (size 0x38) -- masked match 14/14.
 *
 * Two-call helper: calls CallA(this, this->f10, 0, 2, 16) then
 * CallB(this->f0, 31); always returns 1. this->f10 is a byte field at
 * offset 0xa, this->f0 a pointer at offset 0. Sibling function at
 * DllField@0x54d84 has the identical two-call shape but with the calls
 * in the opposite order and different literal arguments (CallB first
 * with (this->f0, 196), then CallA with (this, this->f10, 1, 1, 12)) --
 * these are not a templated-constant family, the call ORDER itself
 * differs per specimen, so each must be read from its own disassembly,
 * not assumed from a sibling.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField \
 *     0x54e08 0x38 xy-decomp/armcc-matching/matched/DllField/0x54e08.c
 */

typedef struct S {
    void *f0;
    unsigned char pad[6];
    unsigned char f10;
} S;

extern long CallA(S *o, long b, long c, long d, long e);
extern long CallB(void *p, long n);

long Func(S *o) {
    CallA(o, o->f10, 0, 2, 16);
    CallB(o->f0, 31);
    return 1;
}
