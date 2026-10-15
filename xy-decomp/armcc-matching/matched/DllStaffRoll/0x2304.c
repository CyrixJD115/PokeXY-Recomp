/* DllStaffRoll function @ CRO file offset 0x2304, size 0x10 — MATCHED
 * (masked, 4/4). Third match in this module. A thin double-forwarder:
 * calls a local function on `this` (masked `bl`), then tail-calls a
 * second function with that call's return value as its argument (masked
 * `b`, no `bl` — lr was already popped before the branch). The `push
 * {r4, lr}` / `pop {r4, lr}` pair is AAPCS stack-alignment padding (r4
 * is never referenced), same idiom already documented for
 * DllDialogCommon@0x6c4.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllStaffRoll 0x2304 0x10 xy-decomp/armcc-matching/matched/DllStaffRoll/0x2304.c
 */

extern long local318(void *this);
extern long veneer318(long v);

long func(void *this)
{
    return veneer318(local318(this));
}
