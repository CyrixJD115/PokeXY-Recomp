/* DllFieldDemo function @ CRO file offset 0x1edc, size 0x8 — MATCHED
 * (masked, 2/2). `xy_system::bg::System::SetIsCallDraw(bool)`: trivial
 * byte-field setter (`strb r1,[r0,#217]; bx lr`) — byte-identical to the
 * same shared base/mixin-class method already matched in
 * DllTrainingAppManager@0xd68, DllPssConfig@0x1c20,
 * DllCaptureRegister@0x27b0 and DllColosseum@0x46fc. Same `+0xd9` field
 * offset recurring across unrelated Dll modules confirms this is one
 * shared base class's method compiled into every Dll that links it, not
 * a per-module coincidence.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllFieldDemo 0x1edc 0x8 \
 *     xy-decomp/armcc-matching/matched/DllFieldDemo/0x1edc.c
 */

typedef struct System { unsigned char pad[217]; unsigned char is_call_draw; } System;

void SetIsCallDraw(System *t, int v)
{
    t->is_call_draw = (unsigned char)v;
}
