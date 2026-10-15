/* DllZukan function @ CRO file offset 0x1be5c, size 0x8 — MATCHED (masked, 2/2).
 * Trivial field getter: returns the long-sized value at this+0xc.
 *
 * Second of the run of three consecutive 2-word prologue-less getters
 * (0x1be54 +0x28, 0x1be5c +0xc, 0x1be64 +0x8).
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllZukan 0x1be5c 0x8 xy-decomp/armcc-matching/matched/DllZukan/0x1be5c.c
 */

typedef struct X {
    unsigned char pad[0xc];
    long fieldC;
} X;

long get_fieldC(X *o)
{
    return o->fieldC;
}
