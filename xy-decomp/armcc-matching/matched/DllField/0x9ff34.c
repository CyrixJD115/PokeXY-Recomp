// DllField@0x9ff34, size 0x24, 9/9 masked match.
// Tail-calls helper on mgr->+12->+72->+240->+2740 (+420) and returns its result.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9ff34.md.
// Style: natural -- plain pointer-chain loads and tail call
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9ff34 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0x9ff34.c

extern char *GetMgr(void);
extern int H(char *);
int Fn(void) {
    char *p = GetMgr();
    return H(*(char **)(*(char **)(*(char **)(*(char **)(p + 12) + 72) + 240) + 2740) + 420);
}
