// DllField@0x9eab8, size 0x24, 9/9 masked match.
// Script-command handler: calls helper on mgr->+12->+100 with args[1]; returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9eab8.md.
// Style: natural -- plain loads and call
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9eab8 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0x9eab8.c

extern char *GetMgr(void);
extern void H(char *, int);
int Fn(int self, int *a) {
    int v = a[1];
    char *p = GetMgr();
    H(*(char **)(*(char **)(p + 12) + 100), v);
    return 0;
}
