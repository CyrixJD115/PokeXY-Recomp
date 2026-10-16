// DllField@0x5c470, size 0x20, 8/8 masked match.
// Gets the manager object from the shared accessor (0x16c0), walks the
// pointer chain +12 -> +72 -> +240, passes the loaded value to a void helper
// and returns 0.
// Shared-accessor thunk family (callee 0x16c0); siblings listed in pending/matched-DllField-0x5c470.md.
// Style: natural -- plain call, pointer-chain loads, helper call; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5c470 0x20 \
//       xy-decomp/armcc-matching/matched/DllField/0x5c470.c

extern char *GetMgr(void);
extern void Sink(int);
int Fn(void) {
    char *p = GetMgr();
    Sink(*(int *)(*(char **)(*(char **)(p + 12) + 72) + 240));
    return 0;
}
