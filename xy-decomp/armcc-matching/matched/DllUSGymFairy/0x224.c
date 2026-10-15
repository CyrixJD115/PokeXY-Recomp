// DllUSGymFairy @0x224, size 0x8 -- trivial virtual-override stub, return 0.
// Match: 2/2 masked words (mov r0,#0 / bx lr).
// One of a run of three identical return-0 stubs at 0x224/0x22c/0x234, sitting
// right after a lone bx-lr stub (0x220) and before two more bx-lr stubs
// (0x23c) and the module's first real function (0x240) -- same recurring
// trivial-stub-cluster shape documented in attempted.md/README.md for every
// other Dll module mined so far (DllUSDynamo@0x1b4, DllUS3DDun@0x24c,
// DllUSBadgeCheckGate@0x228, DllSalon@0xdc8, DllCommWait@0xcd8, ...).
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
//       DllUSGymFairy 0x224 0x8 \
//       xy-decomp/armcc-matching/matched/DllUSGymFairy/0x224.c

int fn_0x224(void *this)
{
    (void)this;
    return 0;
}
