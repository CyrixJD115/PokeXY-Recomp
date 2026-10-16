// DllField@0x9bae8, size 0x24, 9/9 masked match.
// Script-command handler: tail-calls helper on mgr->+12->+72->+408 with args[1] and returns its result.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9bae8.md.
// Style: natural -- plain loads, tail call
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9bae8 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0x9bae8.c

extern char *GetMgr(void);
extern int H(char *, int);
int Fn(int self, int *a) {
    int v = a[1];
    char *p = GetMgr();
    return H(*(char **)(*(char **)(*(char **)(p + 12) + 72) + 408), v);
}
