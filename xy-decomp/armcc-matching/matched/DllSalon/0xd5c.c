/* DllSalon function @ CRO file offset 0xd5c, size 0x38 — MATCHED (masked, 14/14).
 * Style: trick(ordering-local) -- `vt` local only to fix vtable-literal-before-member-load order
 * Destructor-stage: installs vtable, frees member @+0x14 if non-null,
 * tail-calls a second import (identity masked). Same idiom as
 * DllFriendEntry@0x4c4; own literal pool word at 0xd90 => size 0x38.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllSalon 0xd5c 0x38 xy-decomp/armcc-matching/matched/DllSalon/0xd5c.c
 */

typedef struct X {
    void *vptr;              /* +0x00 */
    unsigned char pad[0x10];
    void *buf;               /* +0x14 */
} X;
extern void *VT[];
extern void free2b8(void *p);
extern X *finish1a8(X *o);

X *dtor(X *o)
{
    void *p;
    void *vt = VT;
    p = o->buf;
    o->vptr = vt;
    if (p) {
        free2b8(p);
        o->buf = 0;
    }
    return finish1a8(o);
}
