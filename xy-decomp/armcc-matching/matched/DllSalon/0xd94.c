/* DllSalon function @ CRO file offset 0xd94, size 0x34 — MATCHED (masked, 13/13).
 * Style: natural
 * Destructor-stage: loads member @+0x14, installs vtable, frees member
 * (masked import call) if non-null and zeroes it, returns `this`.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllSalon 0xd94 0x34 xy-decomp/armcc-matching/matched/DllSalon/0xd94.c
 */

typedef struct X {
    void *vptr;              /* +0x00 */
    unsigned char pad[0x10];
    void *buf;               /* +0x14 */
} X;
extern void *VT[];
extern void free2b8(void *p);

X *dtor(X *o)
{
    void *p = o->buf;
    o->vptr = VT;
    if (p) {
        free2b8(p);
        o->buf = 0;
    }
    return o;
}
