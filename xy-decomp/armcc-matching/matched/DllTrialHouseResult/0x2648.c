// DllTrialHouseResult@0x2648 (size 0x38) -- masked match 14/14.
// Style: trick(ordering-local) -- local(s) (e.g. `vt = VT`, hoisted member loads) only to fix load/pool-load order
// Destructor: installs vtable pointer, frees member @+0x14 via a masked
// import-veneer call if non-null, zeroes it, then tail-calls another
// import veneer with `this` (armcc folds the trailing `return Fn(this);`
// into `pop {r4, lr}; b <veneer>` since it's the final statement).
//
// Non-obvious bit: to reproduce the exact instruction order (vtable
// literal load, THEN member load, THEN vtable store, THEN the compare --
// i.e. member-load happens after the vtable-pool-load, matching the
// skill's "good" destructor shape), the vtable literal and the member
// value both need to be pulled into separate named locals up front
// (`void *vt = VT; void *m = a1->member;`) before being used. Writing
// the assignment and the `if` as two plain sequential statements instead
// makes armcc reuse a single register for both the literal and the
// member load (store-then-load, true WAR dependency), which serializes
// them in the wrong order and drops the match to 11/14.
//
// Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 \
//   xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTrialHouseResult \
//   0x2648 0x38 xy-decomp/armcc-matching/matched/DllTrialHouseResult/0x2648.c

typedef struct T {
    void *vptr;
    unsigned char pad[0x10];
    void *member; /* +0x14 */
} T;

extern void *VT2648[];
extern void Free_190(void *p);
extern int TailFn_270(void *a1);

int Fn_2648(T *a1)
{
    void *vt = VT2648;
    void *m = a1->member;
    a1->vptr = vt;
    if (m) {
        Free_190(m);
        a1->member = 0;
    }
    return TailFn_270(a1);
}
