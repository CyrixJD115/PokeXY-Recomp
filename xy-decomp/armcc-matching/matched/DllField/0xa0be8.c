// DllField@0xa0be8, size 0x20, 8/8 masked match.
// Tail-calls H(mgr->+24, mgr->+12->+72->+252) and returns its result.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0xa0be8.md.
// Style: natural -- plain pointer-chain loads and tail call
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xa0be8 0x20 \
//       xy-decomp/armcc-matching/matched/DllField/0xa0be8.c

extern char *GetMgr(void);
extern int H(char *, char *);
int Fn(void) {
    char *p = GetMgr();
    return H(*(char **)(p + 24), *(char **)(*(char **)(*(char **)(p + 12) + 72) + 252));
}
