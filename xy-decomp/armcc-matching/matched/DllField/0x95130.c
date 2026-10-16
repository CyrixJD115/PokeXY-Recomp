// DllField@0x95130, size 0x2c, 11/11 masked match.
// Reads args[1], args[2] (one ldrd), fetches the manager and calls Helper(mgr->+12->+72->+316, a1, a2); returns 0. Sibling of 0x95f78.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x95130 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x95130.c

extern char *GetMgr(void);
extern void Helper(int, int, int);
int Fn(int self, int *a) {
    int x = a[1];
    int y = a[2];
    char *p = GetMgr();
    Helper(*(int *)(*(char **)(*(char **)(p + 12) + 72) + 316), x, y);
    return 0;
}
