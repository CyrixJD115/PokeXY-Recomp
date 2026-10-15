// DllUSGymInsect@0x344, size 0x8: trivial virtual-override stub, return 0.
// Masked match: 2/2 words (b/bl targets masked -- none present here).
//
// Same import-veneer-table-tail stub cluster already documented for
// DllUSDynamo/DllUSBadgeCheckGate/DllUS3DDun/DllUSGymRock/DllUSBigFourWater
// etc. in README.md: a call-through wrapper at 0x318 (push {lr}; sub sp,#12;
// bl veneer@0x180; pop {pc}), then a lone `bx lr` at 0x340, then this
// `mov r0,#0; bx lr` stub, then two more identical `return 0` stubs at
// 0x34c/0x354 (unclaimed), two more lone `bx lr` at 0x35c/0x360, then a
// real function (VFP-heavy dispatcher) at 0x364.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
//       DllUSGymInsect 0x344 0x8 \
//       xy-decomp/armcc-matching/matched/DllUSGymInsect/0x344.c

int fn(void *this_) {
    (void)this_;
    return 0;
}
