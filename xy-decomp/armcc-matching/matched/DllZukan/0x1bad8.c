/* DllZukan function @ CRO file offset 0x1bad8, size 0x14 — MATCHED (masked, 5/5).
 * Boolean getter: this->byte0C == 3.
 * Style: natural
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllZukan 0x1bad8 0x14 xy-decomp/armcc-matching/matched/DllZukan/0x1bad8.c
 */
typedef struct X { unsigned char pad[0xc]; unsigned char field0C; } X;
int is_state3(X *o) { return o->field0C == 3; }
