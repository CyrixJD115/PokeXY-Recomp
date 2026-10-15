/* DllZukan function @ CRO file offset 0x1a3f0, size 0x8 — MATCHED (masked, 2/2).
 * Byte setter: this+0x71 = v.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllZukan 0x1a3f0 0x8 xy-decomp/armcc-matching/matched/DllZukan/0x1a3f0.c
 */
typedef struct X { unsigned char pad[0x71]; unsigned char field71; } X;
void set_field71(X *o, unsigned char v) { o->field71 = v; }
