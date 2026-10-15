/* DllStartMenu @ 0x4c0, size 0x38 — masked match 14/14.
 *
 * One-line: if a check function returns 0, returns 2; else if this->f16==3
 * calls an import veneer on this->f24 with arg 1, then returns this->f16.
 *
 * Note: straightforward control flow, no scheduling surprises. Struct
 * offsets (this+16 int, this+24 pointer) modeled with padding arrays —
 * real member names unknown.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllStartMenu \
 *       0x4c0 0x38 xy-decomp/armcc-matching/matched/DllStartMenu/0x4c0.c
 */
typedef struct T2 {
    unsigned char pad0[16];
    int f16;
    unsigned char pad20[4];
    void *f24;
} T2;

extern int fnC(T2 *p);
extern void fnA(void *p, int x);

int func_4c0(T2 *this)
{
    if (fnC(this) == 0) {
        return 2;
    }
    if (this->f16 == 3) {
        fnA(this->f24, 1);
    }
    return this->f16;
}
