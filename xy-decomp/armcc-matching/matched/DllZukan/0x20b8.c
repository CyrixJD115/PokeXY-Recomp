/* DllZukan function @ CRO file offset 0x20b8, size 0x28 -- MATCHED (masked).
 * Calls virtual slot 0x20 on this->p@0x44, returns this->byte48 != 0.
 * Style: natural
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllZukan 0x20b8 0x28 xy-decomp/armcc-matching/matched/DllZukan/0x20b8.c
 */
typedef struct V { unsigned char pad[0x20]; void (*m)(void *); } V;
typedef struct S { V *vt; } S;
typedef struct X { unsigned char pad[0x44]; S *p; unsigned char f48; } X;
int g(X *o) { o->p->vt->m(o->p); return o->f48 != 0; }
