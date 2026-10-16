// DllField@0x9b950, size 0x3c, 15/15 masked match.
// Script-command handler: calls helper on the sub-object mgr->+12->+72->+408 with args[1] and (args[2] != 0); returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9b950.md.
// Style: natural -- plain loads, bool normalisation, call
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9b950 0x3c \
//       xy-decomp/armcc-matching/matched/DllField/0x9b950.c

extern char *GetMgr(void);
extern void H(char *, int, int);
int Fn(int self, int *a) {
    int f = a[2] != 0;
    int v = a[1];
    char *p = GetMgr();
    H(*(char **)(*(char **)(*(char **)(p + 12) + 72) + 408), v, f);
    return 0;
}
