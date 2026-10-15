/* DllZukan function @ CRO file offset 0x1be64, size 0x8 — MATCHED (masked, 2/2).
 * Trivial field getter: returns the long-sized value at this+0x8.
 *
 * Third of the run of three consecutive 2-word prologue-less getters
 * (0x1be54 +0x28, 0x1be5c +0xc, 0x1be64 +0x8). Immediately followed by
 * an array-indexed getter (`add r0,r0,r1,lsl#2; ldr r0,[r0,#56]; bx lr`)
 * at 0x1be6c (untried — not just a single-field load).
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllZukan 0x1be64 0x8 xy-decomp/armcc-matching/matched/DllZukan/0x1be64.c
 */

typedef struct X {
    unsigned char pad[0x8];
    long field8;
} X;

long get_field8(X *o)
{
    return o->field8;
}
