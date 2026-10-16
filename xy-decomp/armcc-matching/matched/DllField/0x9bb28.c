// DllField@0x9bb28, size 0x24, 9/9 masked match.
// Calls helper on mgr->+12->+72->+408 with constant 1; returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9bb28.md.
// Style: natural -- plain pointer-chain loads and call
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9bb28 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0x9bb28.c

extern char *GetMgr(void);
extern void H(char *, int);
int Fn(void) {
    char *p = GetMgr();
    H(*(char **)(*(char **)(*(char **)(p + 12) + 72) + 408), 1);
    return 0;
}
