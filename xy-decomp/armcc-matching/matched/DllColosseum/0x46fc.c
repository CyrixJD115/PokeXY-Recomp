/* DllColosseum function @ CRO file offset 0x46fc, size 0x8 — MATCHED
 * (masked, 2/2). Confirmed by the module's own named export table:
 * `_ZN9xy_system2bg6System13SetIsCallDrawEb` (xy_system::bg::System::
 * SetIsCallDraw(bool)), load address 0x1046fc. Trivial byte-field setter,
 * `this->is_call_draw = v;`, void return — identical shape (and identical
 * +0xd9 field offset) to the same named export already matched at
 * DllTrainingAppManager@0xd68 and DllPssConfig@0x1c20 (and guessed at
 * DllStrInput@0x666c before the export name was known). A thirty-eighth
 * module confirming this shared base/mixin-class method recurs verbatim
 * across unrelated Dll modules.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllColosseum 0x46fc 0x8 \
 *     xy-decomp/armcc-matching/matched/DllColosseum/0x46fc.c
 */

typedef struct System { unsigned char pad[217]; unsigned char is_call_draw; } System;

void SetIsCallDraw(System *t, int v)
{
    t->is_call_draw = (unsigned char)v;
}
