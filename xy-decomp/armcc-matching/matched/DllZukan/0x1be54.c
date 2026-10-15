/* DllZukan function @ CRO file offset 0x1be54, size 0x8 — MATCHED (masked, 2/2).
 * Trivial field getter: returns the long-sized value at this+0x28.
 *
 * One of a run of three consecutive 2-word prologue-less getters
 * (0x1be54 +0x28, 0x1be5c +0xc, 0x1be64 +0x8) right after a
 * `pop {r4,pc}` epilogue at 0x1be50; followed by an array-indexed
 * getter (`add r0,r0,r1,lsl#2; ldr r0,[r0,#56]; bx lr`) at 0x1be6c.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllZukan 0x1be54 0x8 xy-decomp/armcc-matching/matched/DllZukan/0x1be54.c
 */

typedef struct X {
    unsigned char pad[0x28];
    long field28;
} X;

long get_field28(X *o)
{
    return o->field28;
}
