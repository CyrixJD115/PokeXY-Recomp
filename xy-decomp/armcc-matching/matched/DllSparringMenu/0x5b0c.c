/* DllSparringMenu function @ CRO file offset 0x5b0c, size 0x8 — MATCHED
 * (masked, 2/2). Confirmed by the module's own named export table:
 * `_ZN9xy_system2bg6System13SetIsCallDrawEb` (xy_system::bg::System::
 * SetIsCallDraw(bool)), at file offset 0x5b0c (symbols.tsv load_addr
 * 0x105b0c, base 0x100180 + text-segment offset 0x598c). Trivial byte-field
 * setter,
 * `this->is_call_draw = v;`, void return — identical shape (and identical
 * +0xd9 field offset) to the same named export already matched in many
 * other modules (DllTrainingAppManager@0xd68, DllPssConfig@0x1c20,
 * DllCaptureRegister@0x27b0, DllColosseum@0x46fc, DllFieldDemo@0x1edc,
 * DllIntro@0x9eb0, DllGts@0x1b7e4) — this shared base/mixin-class method
 * recurs verbatim across unrelated Dll modules.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllSparringMenu 0x5b0c 0x8 \
 *     xy-decomp/armcc-matching/matched/DllSparringMenu/0x5b0c.c
 */

typedef struct System { unsigned char pad[217]; unsigned char is_call_draw; } System;

void SetIsCallDraw(System *t, int v)
{
    t->is_call_draw = (unsigned char)v;
}
