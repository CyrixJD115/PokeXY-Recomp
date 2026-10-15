/* DllZukan function @ CRO file offset 0x1d698, size 0x10 — MATCHED (masked, 4/4).
 * Clears a {u32; u16} record.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllZukan 0x1d698 0x10 xy-decomp/armcc-matching/matched/DllZukan/0x1d698.c
 */
typedef struct X { long a; unsigned short b; } X;
void clear(X *o) { o->a = 0; o->b = 0; }
