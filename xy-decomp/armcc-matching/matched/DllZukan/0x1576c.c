/* DllZukan function @ CRO file offset 0x1576c, size 0xc -- MATCHED (masked, 3/3).
 * Setter: this->ptr(+4)->byte[243] = v.
 * Style: natural
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllZukan 0x1576c 0xc xy-decomp/armcc-matching/matched/DllZukan/0x1576c.c
 */

typedef struct In { unsigned char pad[243]; unsigned char f; } In;
typedef struct X { int pad0; In *p; } X;

void set_f(X *o, unsigned char v)
{
    o->p->f = v;
}
