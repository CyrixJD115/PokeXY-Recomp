// DllPssMessageWindow @0x3c0, size 0x8 -- trivial virtual-override stub, return 1.
// Match: 2/2 masked words (mov r0,#1 / bx lr).
// One of a run of five tiny leaf stubs right at the end of the module's
// import-veneer table (which runs 0x180-0x3bf): two identical return-1
// stubs at 0x3c0/0x3c8, then three bare `bx lr` stubs at 0x3d0/0x3d4/0x3d8,
// before the module's first real function (a constructor with VFP math)
// starts at 0x3dc. Same recurring trivial-stub-cluster shape documented in
// README.md/attempted.md for every other Dll module mined so far.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
//       DllPssMessageWindow 0x3c0 0x8 \
//       xy-decomp/armcc-matching/matched/DllPssMessageWindow/0x3c0.c

int fn_0x3c0(void *this)
{
    (void)this;
    return 1;
}
