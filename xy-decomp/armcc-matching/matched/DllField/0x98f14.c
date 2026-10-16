// DllField@0x98f14, size 0x1c, 7/7 masked match.
// Gets the manager object from the shared accessor (0x16c0), loads its +12 sub-object and calls a helper with a constant bool flag argument, returning 0.
// Shared-accessor thunk family (callee 0x16c0); siblings listed in pending/.
// Style: natural -- plain call, pointer-chain loads, helper call; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x98f14 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x98f14.c

extern char *GetMgr(void);
extern void Sink(char *);
extern void Sink2(char *, int);
int Fn(void) {
    char *p = GetMgr();
    Sink2(*(char **)(p + 12), 0);
    return 0;
}
