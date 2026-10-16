/* DllZukan function @ CRO file offset 0x1dcc0, size 0x8 -- MATCHED (masked, 2/2).
 * Returns address of the member at this+0x10 (add r0,r0,#16).
 * Style: natural
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllZukan 0x1dcc0 0x8 xy-decomp/armcc-matching/matched/DllZukan/0x1dcc0.c
 */

typedef struct X { unsigned char pad[0x10]; int m; } X;

int *get_m_addr(X *o)
{
    return &o->m;
}
