/* DllTradelist @ 0xc04, size 0x38 - destructor: installs a new vtable, frees
 * Style: trick(ordering-local) -- local(s) (e.g. `vt = VT`, hoisted member loads) only to fix load/pool-load order
 * the owned member at `this+0x14` if non-null, tail-calls a second import
 * instead of returning `this` directly. Match 14/14.
 *
 * Same idiom as DllFatalErrorProc@0x284 / DllCommendation@0xc94 /
 * DllBattleHouseConnect@0x2fc / DllGameClearSave@0xd6c / DllFriendEntry@0x4c4
 * / DllEshop@0x554 / DllGameSync@0x5e0 / DllLiveCup@0x764: the "good" load
 * ordering (vtable literal into r1 loaded BEFORE the member load into r0,
 * store to o->vptr deferred until after both loads) and the known size
 * gotcha (the function's own one-word literal pool sits at 0xc38, right
 * before the next function's `push {r4,lr}` at 0xc3c, pushing the real
 * size to 0x38, not the naive 0x34).
 *
 * The immediately-following sibling function at 0xc3c has the same shape
 * but with the KNOWN-BAD member-load-before-vtable-load scheduling order
 * (ldr r0,[r0,#20] at 0xc44 comes before ldr r1,[pc,#28] at 0xc48) -
 * recognized on sight and skipped, per the documented residue class.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTradelist \
 *       0xc04 0x38 xy-decomp/armcc-matching/matched/DllTradelist/0xc04.c
 */
typedef struct Obj {
    void *vptr;
    unsigned char pad[16];
    void *member; /* +0x14 */
} Obj;

extern void free_veneer(void *p);
extern void *second_import(void *p);

typedef void VTable;
extern const VTable NewVTable;

void *fn(Obj *o) {
    const VTable *vt = &NewVTable;
    void *m = o->member;
    o->vptr = (void *)vt;
    if (m != 0) {
        free_veneer(m);
        o->member = 0;
    }
    return second_import(o);
}
