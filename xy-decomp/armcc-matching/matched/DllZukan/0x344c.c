/* DllZukan function @ CRO file offset 0x344c, size 0x20 -- MATCHED (masked).
 * Calls import fn(this->p@0x3c), returns this->byte40 != 0.
 * Style: natural
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllZukan 0x344c 0x20 xy-decomp/armcc-matching/matched/DllZukan/0x344c.c
 */
typedef struct X { unsigned char pad[0x3c]; void *p; unsigned char f40; } X;
extern void fn(void *);
int g(X *o) { fn(o->p); return o->f40 != 0; }
