// DllUSGymRock@0x1d4, size 0x8: trivial virtual-override stub, return 0.
// Masked match: 2/2 words (b/bl targets masked -- none present here).
//
// Sits in the module's usual tail-of-import-veneer-table stub cluster: a
// lone `bx lr` at 0x1d0, then three identical `mov r0,#0; bx lr` stubs at
// 0x1d4/0x1dc/0x1e4 (only 0x1d4 claimed here), then two more lone `bx lr`
// at 0x1ec/0x1f0, then a real constructor-shaped function with VFP math
// at 0x1f4 -- the same cluster shape already documented for
// DllUSDynamo/DllUSBadgeCheckGate/DllUS3DDun etc. in README.md.
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
//       DllUSGymRock 0x1d4 0x8 \
//       xy-decomp/armcc-matching/matched/DllUSGymRock/0x1d4.c

int fn(void *this_) {
    (void)this_;
    return 0;
}
