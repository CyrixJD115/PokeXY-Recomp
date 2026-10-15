/* DllField @ 0xe3fc size 0x10 MATCHED (masked 4/4): leaf, zeroes two adjacent int members (this+20, this+24).
 * Style: trick(live-r1-passthrough) -- return type is a 64-bit value built from the two incoming registers
 * Mechanism: armcc puts the shared zero constant in the lowest free scratch register after r0 (=this). Retail
 * used r2, i.e. r1 was live (holding an incoming 2nd argument that is handed back untouched). A plain
 * (void) or unused-parameter version gives `mov r1,#0`. Declaring the function to return a long long
 * made of (this, x) -- low word = r0, high word = r1, zero instructions to build -- keeps r1 live to
 * the end, forcing the zero into r2 with no extra code. Original is probably a thunk/ctor-ish member
 * whose 2nd arg is passed through; exact original signature unknown.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xe3fc 0x10 xy-decomp/armcc-matching/matched/DllField/0xe3fc.c
 */
typedef struct T { unsigned char pad[20]; int a; int b; } T;
long long f(T *t, int x)
{
    t->a = 0;
    t->b = 0;
    return ((long long)x << 32) | (unsigned)t;
}
