// DllField@0x9e344, size 0x1c, 7/7 masked match.
// Gets the manager object from the shared accessor (0x16c0), walks the
// pointer chain +12 -> +100, passes the loaded value to a void helper and
// returns 0.
// Shared-accessor thunk family (callee 0x16c0); siblings listed in pending/matched-DllField-0x9e344.md.
// Style: natural -- plain call, pointer-chain loads, helper call; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9e344 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x9e344.c

extern char *GetMgr(void);
extern void Sink(int);
int Fn(void) {
    char *p = GetMgr();
    Sink(*(int *)(*(char **)(p + 12) + 100));
    return 0;
}
