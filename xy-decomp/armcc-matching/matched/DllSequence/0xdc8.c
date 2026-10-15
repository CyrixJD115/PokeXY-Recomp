/* DllSequence function @ CRO file offset 0xdc8, size 0x8 — MATCHED (masked, 2/2).
 * `xy_system::bg::System::SetIsCallDraw(bool)` — byte-field setter,
 * void return. Identical bytes (`strb r1,[r0,#217]; bx lr`) to the
 * shared base/mixin-class method already matched in several other
 * modules: DllTrainingAppManager@0xd68, DllPssConfig@0x1c20,
 * DllCaptureRegister@0x27b0, DllColosseum@0x46fc, DllFieldDemo@0x1edc,
 * DllIntro@0x9eb0 — same `+0xd9` field, same shape, recurring across
 * unrelated modules.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllSequence 0xdc8 0x8 xy-decomp/armcc-matching/matched/DllSequence/0xdc8.c
 */

struct System
{
    unsigned char pad[217];
    unsigned char isCallDraw;
};

void SetIsCallDraw(struct System *this, int v)
{
    this->isCallDraw = (unsigned char)v;
}
