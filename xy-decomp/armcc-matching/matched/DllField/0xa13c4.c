// DllField@0xa13c4, size 0x20, 8/8 masked match.
// Script-command handler: calls helper on mgr->+24 with args[1]; returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0xa13c4.md.
// Style: natural -- plain loads and call
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xa13c4 0x20 \
//       xy-decomp/armcc-matching/matched/DllField/0xa13c4.c

extern char *GetMgr(void);
extern void H(char *, int);
int Fn(int self, int *a) {
    int v = a[1];
    char *p = GetMgr();
    H(*(char **)(p + 24), v);
    return 0;
}
