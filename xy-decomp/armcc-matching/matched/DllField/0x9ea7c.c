// DllField@0x9ea7c, size 0x18, 6/6 masked match.
// Gets the manager object from the shared accessor (0x16c0), walks the
// pointer chain +12 -> +100 -> +2712 and returns the loaded word.
// Shared-accessor thunk family (callee 0x16c0); siblings listed in pending/matched-DllField-0x9ea7c.md.
// Style: natural -- plain call, pointer-chain loads, helper call; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9ea7c 0x18 \
//       xy-decomp/armcc-matching/matched/DllField/0x9ea7c.c

extern char *GetMgr(void);
int Fn(void) {
    char *p = GetMgr();
    return *(int *)(*(char **)(*(char **)(p + 12) + 100) + 2712);
}
