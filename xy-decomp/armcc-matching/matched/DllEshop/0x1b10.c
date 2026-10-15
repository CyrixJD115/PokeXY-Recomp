/* DllEshop function @ CRO file offset 0x1b10, size 0x2c — MATCHED (masked, 11/11).
 * Style: trick(static-helper) -- static one-use helper only to reproduce retail's register reuse
 * Calls an import with (2,0,8,0), sets byte flag this+0x58 = 1, returns 22
 * (a state-machine step result). Non-obvious: writing the store inline
 * picks r1 for the constant; going through a static setter taking `int v`
 * reuses r0 as retail does.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllEshop 0x1b10 0x2c xy-decomp/armcc-matching/matched/DllEshop/0x1b10.c
 */
typedef struct X { unsigned char pad[0x58]; unsigned char f58; } X;
extern void imp368(int a, int b, int c, int d);
static void set(X *o, int v) { o->f58 = v; }
int fn(X *o)
{
    imp368(2, 0, 8, 0);
    set(o, 1);
    return 22;
}
