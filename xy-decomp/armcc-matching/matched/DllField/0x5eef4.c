// DllField@0x5eef4, size 0x24, 9/9 masked match.
// Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +404; if the loaded pointer is non-null calls a helper with it (conditional `blne`), returns 0.
// Shared-accessor thunk family (callee 0x16c0); siblings listed in pending/.
// Style: natural -- plain call, pointer-chain loads, helper call; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5eef4 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0x5eef4.c

extern char *GetMgr(void);
extern void Sink(char *);
int Fn(void) {
    char *p = GetMgr();
    char *v = *(char **)(*(char **)(*(char **)(p + 12) + 72) + 404);
    if (v != 0) Sink(v);
    return 0;
}
