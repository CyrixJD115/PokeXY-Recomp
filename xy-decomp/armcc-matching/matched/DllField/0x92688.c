// DllField@0x92688, size 0x2c, 11/11 masked match.
// Reads args[1], fetches the manager and calls Helper(mgr->+12->+28->+36, args[1], 10); returns 0.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads, helper call; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x92688 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x92688.c

extern char *GetMgr(void);
extern void Helper(int, int, int);
int Fn(int self, int *a) {
    int v = a[1];
    char *p = GetMgr();
    Helper(*(int *)(*(char **)(*(char **)(p + 12) + 28) + 36), v, 10);
    return 0;
}
