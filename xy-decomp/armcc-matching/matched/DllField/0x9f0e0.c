// DllField@0x9f0e0, size 0x20, 8/8 masked match.
// Script-command handler: tail-calls helper on mgr->+12->+100 with args[1] and returns its result.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9f0e0.md.
// Style: natural -- plain loads and tail call
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9f0e0 0x20 \
//       xy-decomp/armcc-matching/matched/DllField/0x9f0e0.c

extern char *GetMgr(void);
extern int H(char *, int);
int Fn(int self, int *a) {
    int v = a[1];
    char *p = GetMgr();
    return H(*(char **)(*(char **)(p + 12) + 100), v);
}
