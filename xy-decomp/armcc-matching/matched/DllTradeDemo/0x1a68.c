// DllTradeDemo@0x1a68, size 0x8, masked match 2/2.
// `xy_system::bg::System::SetIsCallDraw(bool)` -- byte-field setter at
// offset 217 (0xd9), void return. Identical bytes to
// DllTrainingAppManager@0xd68/DllPssConfig@0x1c20/
// DllCaptureRegister@0x27b0/DllColosseum@0x46fc/DllFieldDemo@0x1edc/
// DllIntro@0x9eb0: the same shared base/mixin-class method recurring
// across unrelated modules.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
//     DllTradeDemo 0x1a68 0x8 \
//     xy-decomp/armcc-matching/matched/DllTradeDemo/0x1a68.c

typedef struct {
    unsigned char pad[217];
    unsigned char is_call_draw;
} System;

void SetIsCallDraw(System *s, int v) {
    s->is_call_draw = (unsigned char)v;
}
