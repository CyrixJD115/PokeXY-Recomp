/* DllGts function @ CRO file offset 0x80f0, size 0x3c -- MATCHED (masked) 15/15.
 * Sibling of 0x7e50: fn(this->c@0x20, this+0x24, 4, 0, this->p@0xc->byte@0x227, 0); returns 5.
 * Style: natural
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllGts 0x80f0 0x3c xy-decomp/armcc-matching/matched/DllGts/0x80f0.c
 */
typedef struct P { unsigned char pad[0x227]; unsigned char b; } P;
typedef struct X { unsigned char pad[0xc]; P *p; unsigned char pad2[0x10]; void *c; unsigned char buf[4]; } X;
extern void fn(void *, void *, int, int, int, int);
int g(X *o) { fn(o->c, o->buf, 4, 0, o->p->b, 0); return 5; }
