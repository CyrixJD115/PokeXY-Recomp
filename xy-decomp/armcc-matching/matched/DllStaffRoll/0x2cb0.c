/* DllStaffRoll function @ CRO file offset 0x2cb0, size 0x8 — MATCHED
 * (masked, 2/2). Fifth match in this module.
 * `xy_system::bg::System::SetIsCallDraw(bool)` — byte-field setter,
 * identical bytes to DllTrainingAppManager@0xd68/DllPssConfig@0x1c20/
 * DllCaptureRegister@0x27b0/DllColosseum@0x46fc/DllFieldDemo@0x1edc/
 * DllIntro@0x9eb0: the same shared base/mixin-class method recurring
 * across unrelated modules. Sits right after the dtor at 0x2c44's
 * known-bad-ordering sibling (0x2c7c, skipped); an untried `return 0`
 * stub immediately follows at 0x2cb8.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllStaffRoll 0x2cb0 0x8 xy-decomp/armcc-matching/matched/DllStaffRoll/0x2cb0.c
 */

typedef struct System { unsigned char pad[217]; unsigned char is_call_draw; } System;

void SetIsCallDraw(System *t, int v)
{
    t->is_call_draw = (unsigned char)v;
}
