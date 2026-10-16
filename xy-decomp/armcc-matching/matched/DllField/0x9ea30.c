// DllField@0x9ea30, size 0x1c, 7/7 masked match.
// Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +100 -> +2916, adds a field offset and returns the signed byte (`ldrsb`) found there.
// Shared-accessor thunk family (callee 0x16c0); siblings listed in pending/.
// Style: natural -- plain call, pointer-chain loads, helper call; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9ea30 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x9ea30.c

extern char *GetMgr(void);
extern void Sink(char *);
int Fn(void) {
    char *p = GetMgr();
    return *(signed char *)(*(char **)(*(char **)(p + 12) + 100) + 2916);
}
