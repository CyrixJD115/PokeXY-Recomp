/* DllIntro function @ CRO file offset 0x9eb0, size 0x8 — MATCHED
 * (masked, 2/2). This is the module's own named export
 * `_ZN9xy_system2bg6System13SetIsCallDrawEb` (xy_system::bg::System::
 * SetIsCallDraw(bool)): trivial byte-field setter, `this->is_call_draw = v;`,
 * void return. Identical bytes to the same shared base/mixin-class setter
 * already matched in DllTrainingAppManager@0xd68 / DllPssConfig@0x1c20 /
 * DllColosseum@0x46fc / DllCaptureRegister@0x27b0 — confirmed here via
 * the module's own named export table, same +0xd9 field offset.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllIntro 0x9eb0 0x8 \
 *     xy-decomp/armcc-matching/matched/DllIntro/0x9eb0.c
 * Style: natural
 */

typedef struct System { unsigned char pad[217]; unsigned char is_call_draw; } System;

void SetIsCallDraw(System *t, int v)
{
    t->is_call_draw = (unsigned char)v;
}
