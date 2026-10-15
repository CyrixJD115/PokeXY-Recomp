/* DllZukan function @ CRO file offset 0x1a1a4, size 0x8 — MATCHED (masked, 2/2).
 * Trivial field getter: returns the long at this+0x20.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllZukan 0x1a1a4 0x8 xy-decomp/armcc-matching/matched/DllZukan/0x1a1a4.c
 */
typedef struct X { unsigned char pad[0x20]; long field20; } X;
long get_field20(X *o) { return o->field20; }
