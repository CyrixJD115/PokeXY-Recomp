/* DllPssConfig function @ CRO file offset 0x1c20, size 0x8 — MATCHED
 * (masked, 2/2). This is the module's own named export
 * `_ZN9xy_system2bg6System13SetIsCallDrawEb` (xy_system::bg::System::
 * SetIsCallDraw(bool)) — same shared base/mixin class method already
 * matched at DllTrainingAppManager@0xd68, confirming the twenty-second-
 * plus module recurrence of this trivial byte-field setter at the same
 * +0xd9 offset: `this->is_call_draw = v;`, void return.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllPssConfig 0x1c20 0x8 \
 *     xy-decomp/armcc-matching/matched/DllPssConfig/0x1c20.c
 * Style: natural
 */

typedef struct System { unsigned char pad[217]; unsigned char is_call_draw; } System;

void SetIsCallDraw(System *t, int v)
{
    t->is_call_draw = (unsigned char)v;
}
