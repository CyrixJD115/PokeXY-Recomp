// DllUSGymElec@0x268, size 0x8: trivial virtual-override stub, return 0.
// Masked match: 2/2 words (b/bl targets masked -- none present here).
//
// Sits in the module's usual tail-of-import-veneer-table stub cluster:
// 28 import veneers (0x180-0x25f), then two lone `bx lr` weak-evidence
// stubs at 0x260/0x264, then three identical `mov r0,#0; bx lr` stubs at
// 0x268/0x270/0x278 (only 0x268 claimed here), then two more lone `bx lr`
// at 0x280/0x284, then the module's first real function (a field getter
// with an early-out comparison) at 0x288. Same recurring cluster shape
// documented for DllUSGymRock/DllUSGymGrass/DllUSGymFairy/DllUSGymInsect
// etc. in README.md.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
//       DllUSGymElec 0x268 0x8 \
//       xy-decomp/armcc-matching/matched/DllUSGymElec/0x268.c

int fn(void *this_) {
    (void)this_;
    return 0;
}
