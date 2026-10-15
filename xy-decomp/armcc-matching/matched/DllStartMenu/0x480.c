/* DllStartMenu @ 0x480, size 0x40 — masked match 16/16.
 *
 * One-line: init helper — calls an import veneer on this->f24 with arg 0,
 * sets this->f34 = 1 and this->f4->f2 = 1, then tail-calls a second veneer
 * with constant args (0x60065, 0, -1, 0), returning its result directly.
 *
 * Note: the tail call ("pop {r4,lr}; mov r1,r3; b <veneer>") reuses r2=-1
 * and r3=0 that were materialized early (right after the first bl) even
 * though they're dead until the very last instruction — armcc's scheduler
 * filled otherwise-idle slots around the middle of the function with the
 * final call's constant-argument setup. Writing the C in natural source
 * order (call, two stores, then `return fnB(...)`) reproduced this
 * scheduling exactly; no manual reordering was needed. Struct offsets
 * (this+4 pointer, this+24 pointer, this+52 byte, target+2 byte) modeled
 * with padding arrays — real member names unknown.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllStartMenu \
 *       0x480 0x40 xy-decomp/armcc-matching/matched/DllStartMenu/0x480.c
 */
typedef struct Sub { unsigned char pad0[2]; unsigned char f2; } Sub;
typedef struct T {
    unsigned char pad0[4];
    Sub *f4;
    unsigned char pad8[16];
    void *f24;
    unsigned char pad28[24];
    unsigned char f34;
} T;

extern void fnA(void *p, int x);
extern int fnB(int a, int b, int c, int d);

int func_480(T *this)
{
    fnA(this->f24, 0);
    this->f34 = 1;
    this->f4->f2 = 1;
    return fnB(0x60065, 0, -1, 0);
}
