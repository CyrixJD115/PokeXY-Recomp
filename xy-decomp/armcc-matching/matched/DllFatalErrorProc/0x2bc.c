/* DllFatalErrorProc function @ CRO file offset 0x2bc, size 0x34 — MATCHED (masked, 13/13). First match from a fourth module. Destructor: vptr install, free member @+0x14, returns this (ARM C++ ABI) — structurally identical to DllDialogCommon@0xcdc, confirming this dtor idiom recurs verbatim across modules.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllFatalErrorProc 0x2bc 0x34 xy-decomp/armcc-matching/matched/DllFatalErrorProc/0x2bc.c
 */

typedef struct X {
    void *vptr;              /* +0x00 */
    unsigned char pad[0x10];
    void *buf;               /* +0x14 */
} X;
extern void *VT[];           /* vtable, pool literal */
extern void free218(void *p);

X *dtor(X *o)
{
    void *p = o->buf;
    o->vptr = VT;
    if (p) {
        free218(p);
        o->buf = 0;
    }
    return o;
}
