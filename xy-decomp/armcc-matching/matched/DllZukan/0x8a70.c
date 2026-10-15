/* DllZukan function @ CRO file offset 0x8a70, size 0x24 -- MATCHED (masked).
 * If this->p@0xd0 non-null, calls its vtable slot 1 passing (p, a); returns 0.
 * Style: natural
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllZukan 0x8a70 0x24 xy-decomp/armcc-matching/matched/DllZukan/0x8a70.c
 */
typedef struct V { unsigned char pad[4]; void (*m)(void *, int); } V;
typedef struct S { V *vt; } S;
typedef struct X { unsigned char pad[0xd0]; S *p; } X;
int g(X *o, int a) { S *p = o->p; if (p) p->vt->m(p, a); return 0; }
