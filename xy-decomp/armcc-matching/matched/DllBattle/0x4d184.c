/*
 * DllBattle @ 0x4d184, size 0x44 -- range-checked hi-lo getter: asserts (fnA(0,0,0)) when i >= o->count (u8 at +338), then returns byte(+287+14i) - byte(+286+14i) (hi minus lo of a 14-byte-stride record; sibling of 0x520e4).
 * Masked match 17/17.
 * Style: trick(ordering-local) -- local `lo` forces the retail load order (lo byte first; plain `hi - lo` loads hi first). Compiles as plain C (the old .cpp was only for declaration-after-statement).
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x4d184 0x44 xy-decomp/armcc-matching/matched/DllBattle/0x4d184.c
 */
extern void fnA(int a, int b, int c);
struct Q { char pad[338]; unsigned char n; };
unsigned char fn(struct Q *o, unsigned i) {
    unsigned char lo;
    if (!(i < o->n))
        fnA(0,0,0);
    lo = ((unsigned char *)o)[286 + i*14];
    return ((unsigned char *)o)[287 + i*14] - lo;
}
