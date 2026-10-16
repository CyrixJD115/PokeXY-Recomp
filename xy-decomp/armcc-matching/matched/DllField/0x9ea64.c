// DllField@0x9ea64, size 0x18, 6/6 masked match.
// Gets the manager object from the shared accessor (0x16c0), walks the
// pointer chain +12, passes the loaded value to a void helper and returns 0.
// Shared-accessor thunk family (callee 0x16c0); siblings listed in pending/matched-DllField-0x9ea64.md.
// Style: natural -- plain call, pointer-chain loads, helper call; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9ea64 0x18 \
//       xy-decomp/armcc-matching/matched/DllField/0x9ea64.c

extern char *GetMgr(void);
extern void Sink(int);
int Fn(void) {
    char *p = GetMgr();
    Sink(*(int *)(p + 12));
    return 0;
}
