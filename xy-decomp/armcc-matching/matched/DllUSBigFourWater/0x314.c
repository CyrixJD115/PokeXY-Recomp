// DllUSBigFourWater@0x314, size 0x8: trivial virtual-override stub, return 0.
// Masked match: 2/2 words (b/bl targets masked -- none present here).
//
// Same import-veneer-table-tail stub cluster already documented for
// DllUSDynamo/DllUSBadgeCheckGate/DllUS3DDun/DllUSGymRock etc. in
// README.md: a lone `bx lr` at 0x310, then three identical
// `mov r0,#0; bx lr` stubs at 0x314/0x31c/0x324 (only 0x314 claimed
// here), then one more lone `bx lr` at 0x32c, then a real function at
// 0x330.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
//       DllUSBigFourWater 0x314 0x8 \
//       xy-decomp/armcc-matching/matched/DllUSBigFourWater/0x314.c

int fn(void *this_) {
    (void)this_;
    return 0;
}
