// DllField@0xa0cd8, size 0x1c, 7/7 masked match.
// Script-command handler: tail-calls helper on mgr->+44 with args[1] and returns its result (byte-identical sibling of 0xa0008).
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0xa0cd8.md.
// Style: natural -- plain loads and tail call
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xa0cd8 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0xa0cd8.c

extern char *GetMgr(void);
extern int H(char *, int);
int Fn(int self, int *a) {
    int v = a[1];
    char *p = GetMgr();
    return H(*(char **)(p + 44), v);
}
