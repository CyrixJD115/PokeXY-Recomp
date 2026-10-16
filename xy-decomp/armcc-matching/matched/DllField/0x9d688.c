// DllField@0x9d688, size 0x2c, 11/11 masked match.
// Calls F(8), then tail-calls H(mgr->+12->+28, F's result).
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9d688.md.
// Style: natural -- plain call, member load, tail call
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9d688 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x9d688.c

extern char *GetMgr(void);
extern int F(int);
extern int H(char *, int);
int Fn(void) {
    char *p = GetMgr();
    int r = F(8);
    return H(*(char **)(*(char **)(p + 12) + 28), r);
}
