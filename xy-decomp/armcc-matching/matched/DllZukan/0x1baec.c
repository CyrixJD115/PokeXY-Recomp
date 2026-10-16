/* DllZukan function @ CRO file offset 0x1baec, size 0x8 -- MATCHED (masked, 2/2).
 * Getter: returns word at this+0x74.
 * Style: natural
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllZukan 0x1baec 0x8 xy-decomp/armcc-matching/matched/DllZukan/0x1baec.c
 */

typedef struct X { unsigned char pad[0x74]; int f; } X;

int get_f74(X *o)
{
    return o->f;
}
