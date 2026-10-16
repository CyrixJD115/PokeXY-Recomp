/* DllStaffRoll function @ CRO file offset 0xe90, size 0x10 — MATCHED
 * (masked, 4/4). Fourth match in this module. Calls a function on
 * `this` (masked `bl`, return value discarded) and unconditionally
 * returns 1 via `pop {r4, pc}`. `push {r4, lr}`/`pop {r4, pc}` is a
 * normal stack frame (needed because the body makes a real `bl` call,
 * unlike the AAPCS-padding-only variants).
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllStaffRoll 0xe90 0x10 xy-decomp/armcc-matching/matched/DllStaffRoll/0xe90.c
 * Style: natural
 */

extern void helper90(void *this);

long func(void *this)
{
    helper90(this);
    return 1;
}
