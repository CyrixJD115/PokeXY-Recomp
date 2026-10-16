// DllField@0x59e0c, size 0x30, 12/12 masked match.
// Reads the event argument word a[1] first, gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0, adds a field offset and calls a helper with (address, a[1]), returning 0.
// Shared-accessor thunk family (callee 0x16c0); siblings listed in pending/.
// Style: natural -- plain call, pointer-chain loads, helper call; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x59e0c 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0x59e0c.c

extern char *GetMgr(void);
extern void Sink(char *);
extern void Sink2(char *, int);
int Fn(int self, int *a) {
    int x = a[1];
    char *p = GetMgr();
    Sink2(*(char **)(*(char **)(*(char **)(p + 12) + 28) + 0) + 13328, x);
    return 0;
}
