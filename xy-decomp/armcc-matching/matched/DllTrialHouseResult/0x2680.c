// DllTrialHouseResult@0x2680 (size 0x34) -- masked match 13/13.
// Destructor: frees member @+0x14 via a masked import-veneer call if
// non-null, zeroes it, installs the vtable pointer, returns `this`
// (ARM C++ ABI). Unlike the sibling @0x2648 destructor in this module,
// this one returns `this` directly (no tail call), and the retail
// instruction order is member-load, vtable-literal-load, compare,
// vtable-store -- reproduced simply by fetching the member into a plain
// local (`void *m = a1->member;`) before the vtable assignment and the
// `if`, same shape as the already-matched DllDialogCommon@0xcdc
// destructor.
// Style: natural
//
// Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 \
//   xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTrialHouseResult \
//   0x2680 0x34 xy-decomp/armcc-matching/matched/DllTrialHouseResult/0x2680.c

typedef struct T {
    void *vptr;
    unsigned char pad[0x10];
    void *member; /* +0x14 */
} T;

extern void *VT2680[];
extern void Free_1c0(void *p);

T *Fn_2680(T *a1)
{
    void *m = a1->member;
    a1->vptr = VT2680;
    if (m) {
        Free_1c0(m);
        a1->member = 0;
    }
    return a1;
}
