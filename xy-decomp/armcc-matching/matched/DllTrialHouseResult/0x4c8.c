// DllTrialHouseResult@0x4c8 (size 0x1c) -- masked match 7/7.
// Simple guarded field accessor: calls an import veneer on `this`
// (likely a validity/ready check), returns `this+28` if the call result
// is non-zero, else returns the constant 2 (probably a "not ready"
// status code).
// Style: natural
//
// Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 \
//   xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTrialHouseResult \
//   0x4c8 0x1c xy-decomp/armcc-matching/matched/DllTrialHouseResult/0x4c8.c

extern int fn180(void *a1);

int Fn_4c8(void *a1)
{
    if (fn180(a1) != 0)
        return *(int *)((char *)a1 + 28);
    return 2;
}
