/* DllZukan function @ CRO file offset 0x156c4, size 0xc -- MATCHED (masked, 3/3).
 * Setter: this->ptr(+4)->byte[232] = v.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllZukan 0x156c4 0xc xy-decomp/armcc-matching/matched/DllZukan/0x156c4.c
 */

typedef struct In { unsigned char pad[232]; unsigned char f; } In;
typedef struct X { int pad0; In *p; } X;

void set_f(X *o, unsigned char v)
{
    o->p->f = v;
}
