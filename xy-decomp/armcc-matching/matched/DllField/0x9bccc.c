// DllField@0x9bccc, size 0x1c, 7/7 masked match.
// Gets the manager object from the shared accessor (0x16c0), walks the
// pointer chain +12 -> +72 -> +408, and tail-calls a helper with the loaded
// value (`pop {r4,lr}; b`).
// Shared-accessor thunk family (callee 0x16c0); siblings listed in pending/matched-DllField-0x9bccc.md.
// Style: natural -- plain call, pointer-chain loads, helper call; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9bccc 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x9bccc.c

extern char *GetMgr(void);
extern int Tail(int);
int Fn(void) {
    char *p = GetMgr();
    return Tail(*(int *)(*(char **)(*(char **)(p + 12) + 72) + 408));
}
