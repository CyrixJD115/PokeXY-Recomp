/* DllField @ 0xe1fe8 size 0x2c MATCHED (masked 11/11): `f11(a->first); return f79(a, a->sub.y, 0, 5);` (sub-object at +16, byte at +17).
 * Retail ends in a NON-tail call (`bl; pop {r4,r5,r6,pc}`) returning the callee's r0.
 * Style: trick(wider-callee-return) -- the callee is declared `long long` and the caller returns `(int)` of it. The
 *   narrowing cast needs no code (r0 is the low word) but makes armcc treat the call as non-tail (a plain
 *   `return f(...)` with matching types becomes `pop {..,lr}; b f`, 6/11). Twin of 0xe1c4c.
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xe1fe8 0x2c xy-decomp/armcc-matching/matched/DllField/0xe1fe8.c
 */
struct Sub { unsigned char x, y; };
struct A { void *a; char pad[12]; struct Sub s; };
extern void *f11(void *);
extern long long f79(void *, int, int, int);
int f(void *u, struct A *a)
{
    struct Sub *p = &a->s;
    f11(a->a);
    return (int)f79(a, p->y, 0, 5);
}
