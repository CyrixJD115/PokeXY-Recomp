// DllField@0x9adc4, size 0x1c, 7/7 masked match.
// Gets the manager object from the shared accessor (0x16c0), walks the
// pointer chain +12 -> +28 -> +56 -> +0 and returns the loaded word.
// Shared-accessor thunk family (callee 0x16c0); siblings listed in pending/matched-DllField-0x9adc4.md.
// Style: natural -- plain call, pointer-chain loads, helper call; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9adc4 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x9adc4.c

extern char *GetMgr(void);
int Fn(void) {
    char *p = GetMgr();
    return *(int *)(*(char **)(*(char **)(*(char **)(p + 12) + 28) + 56) + 0);
}
