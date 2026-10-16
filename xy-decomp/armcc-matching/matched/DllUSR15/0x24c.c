// DllUSR15@0x24c, size 0x8: trivial virtual-override stub, return 0.
// Masked match: 2/2 words (b/bl targets masked -- none present here).
//
// Same import-veneer-table-tail stub cluster already documented for
// DllUSDynamo/DllUSBadgeCheckGate/DllUS3DDun/DllUSGymRock/
// DllUSBigFourWater etc. in README.md: the import-veneer table runs
// 0x180-0x247, then a lone `bx lr` at 0x248, three identical
// `mov r0,#0; bx lr` stubs at 0x24c/0x254/0x25c (only 0x24c claimed
// here), then three more lone `bx lr` at 0x260/0x264/0x268, then a
// real (VFP-using) function at 0x26c.
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
//       DllUSR15 0x24c 0x8 \
//       xy-decomp/armcc-matching/matched/DllUSR15/0x24c.c

int fn(void *this_) {
    (void)this_;
    return 0;
}
