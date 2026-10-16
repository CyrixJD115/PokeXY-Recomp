// DllField@0x9d564, size 0x44, 17/17 masked match.
// Script-command handler: if mgr->+12->+72->+408 is non-null tail-calls H(o, args[1]); else E(0,0,0) and returns 1.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9d564.md.
// Style: natural -- plain null test, tail call, fallthrough call
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9d564 0x44 \
//       xy-decomp/armcc-matching/matched/DllField/0x9d564.c

extern char *GetMgr(void);
extern int H(char *, int);
extern void E(int, int, int);
int Fn(int self, int *a) {
    int v = a[1];
    char *o = *(char **)(*(char **)(*(char **)(GetMgr() + 12) + 72) + 408);
    if (o) return H(o, v);
    E(0, 0, 0);
    return 1;
}
