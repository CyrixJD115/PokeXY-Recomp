// DllField@0x98d14, size 0x28, 10/10 masked match.
// Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +85200, adds a constant field offset and passes the resulting address to a void helper, returning 0.
// Shared-accessor thunk family (callee 0x16c0); siblings listed in pending/.
// Style: natural -- plain call, pointer-chain loads, helper call; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x98d14 0x28 \
//       xy-decomp/armcc-matching/matched/DllField/0x98d14.c

extern char *GetMgr(void);
extern void Sink(char *);
int Fn(void) {
    char *p = GetMgr();
    Sink(*(char **)(*(char **)(*(char **)(p + 12) + 28) + 0) + 85200);
    return 0;
}
