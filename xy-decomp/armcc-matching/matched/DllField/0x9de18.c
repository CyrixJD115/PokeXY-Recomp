// DllField@0x9de18, size 0x28, 10/10 masked match.
// Reads the event argument word a[1] first, gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +248 and calls a helper with (loaded object, a[1]), returning 0.
// Shared-accessor thunk family (callee 0x16c0); siblings listed in pending/.
// Style: natural -- plain call, pointer-chain loads, helper call; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9de18 0x28 \
//       xy-decomp/armcc-matching/matched/DllField/0x9de18.c

extern char *GetMgr(void);
extern void Sink(char *);
extern void Sink2(char *, int);
int Fn(int self, int *a) {
    int x = a[1];
    char *p = GetMgr();
    Sink2(*(char **)(*(char **)(*(char **)(p + 12) + 72) + 248), x);
    return 0;
}
