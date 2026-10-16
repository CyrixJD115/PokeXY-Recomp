// DllField@0x9feb4, size 0x28, 10/10 masked match.
// Calls helper on the animation sub-object mgr->+12->+72->+240->+2740 (+724); returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9feb4.md.
// Style: natural -- plain pointer-chain loads and call
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9feb4 0x28 \
//       xy-decomp/armcc-matching/matched/DllField/0x9feb4.c

extern char *GetMgr(void);
extern void H(char *);
int Fn(void) {
    char *p = GetMgr();
    H(*(char **)(*(char **)(*(char **)(*(char **)(p + 12) + 72) + 240) + 2740) + 724);
    return 0;
}
