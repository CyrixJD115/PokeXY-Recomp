/* DllTrainingAppManager function @ CRO file offset 0xd68, size 0x8 — MATCHED
 * (masked, 2/2). This is the module's own named export
 * `_ZN9xy_system2bg6System13SetIsCallDrawEb` (xy_system::bg::System::
 * SetIsCallDraw(bool)): trivial byte-field setter, `this->is_call_draw = v;`,
 * void return. Same shape as DllStrInput@0x666c (field at the same +0xd9
 * offset) — the named export here confirms what that one could only guess
 * at: this is a `bool`-typed flag setter on a shared base/mixin class, not
 * an arbitrary byte field.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTrainingAppManager 0xd68 0x8 \
 *     xy-decomp/armcc-matching/matched/DllTrainingAppManager/0xd68.c
 */

typedef struct System { unsigned char pad[217]; unsigned char is_call_draw; } System;

void SetIsCallDraw(System *t, int v)
{
    t->is_call_draw = (unsigned char)v;
}
