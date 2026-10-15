// DllTradeDemo@0x1a70, size 0x8, masked match 2/2.
// Trivial virtual-override stub, `return 1`, ignores `this`. Sits right
// after the SetIsCallDraw setter at 0x1a68.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
//     DllTradeDemo 0x1a70 0x8 \
//     xy-decomp/armcc-matching/matched/DllTradeDemo/0x1a70.c

typedef struct Obj Obj;

int AlwaysTrue(Obj *self) {
    (void)self;
    return 1;
}
