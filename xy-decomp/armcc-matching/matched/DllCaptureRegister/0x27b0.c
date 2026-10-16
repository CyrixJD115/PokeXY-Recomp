/* DllCaptureRegister function @ CRO file offset 0x27b0, size 0x8 — MATCHED
 * (masked, 2/2). Same `xy_system::bg::System::SetIsCallDraw(bool)` byte-
 * field setter already proven at DllTrainingAppManager@0xd68 and
 * DllPssConfig@0x1c20 (identical bytes: `strb r1, [r0, #217]; bx lr`) — a
 * shared base/mixin-class method reused verbatim across unrelated modules,
 * this being the thirty-ninth module touched by the swarm.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllCaptureRegister 0x27b0 0x8 \
 *     xy-decomp/armcc-matching/matched/DllCaptureRegister/0x27b0.c
 */

typedef struct System { unsigned char pad[217]; unsigned char is_call_draw; } System;

void SetIsCallDraw(System *t, int v)
{
    t->is_call_draw = (unsigned char)v;
}
