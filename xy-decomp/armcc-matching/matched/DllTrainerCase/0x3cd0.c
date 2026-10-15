/* DllTrainerCase function @ CRO file offset 0x3cd0, size 0x8 — MATCHED
 * (masked, 2/2). Tenth match in this module.
 * Trivial sub-object accessor: `add r0,r0,#28; bx lr` — returns a pointer
 * to a member embedded at `this+28`, pure pointer arithmetic, no load.
 * Same idiom as the already-matched `0x3cc0` (`+16`) and `0x3cc8` (`+4`);
 * second of the three near-identical siblings flagged as a leftover
 * lead (`0x3cc8` +4, `0x3cd0` +28, `0x3cd8` +8).
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTrainerCase 0x3cd0 0x8 xy-decomp/armcc-matching/matched/DllTrainerCase/0x3cd0.c
 */

typedef struct X {
    unsigned char pad[0x1c];
    unsigned char member[4]; /* +0x1c, embedded sub-object */
} X;

void *get_member(X *this)
{
    return &this->member;
}
