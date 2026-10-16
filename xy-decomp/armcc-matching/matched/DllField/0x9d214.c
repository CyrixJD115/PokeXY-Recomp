// DllField@0x9d214, size 0x44, 17/17 masked match.
// Script-command handler: if mgr->+12->+72->+408 is non-null calls H(o, args[1]), else E(0,0,0); returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9d214.md.
// Style: natural -- plain null test and two calls
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9d214 0x44 \
//       xy-decomp/armcc-matching/matched/DllField/0x9d214.c

extern char *GetMgr(void);
extern void H(char *, int);
extern void E(int, int, int);
int Fn(int self, int *a) {
    int v = a[1];
    char *o = *(char **)(*(char **)(*(char **)(GetMgr() + 12) + 72) + 408);
    if (o) H(o, v); else E(0, 0, 0);
    return 0;
}
