// DllField@0x9f188, size 0x28, 10/10 masked match.
// Returns whether a helper on mgr->+12->+100 returns nonzero (result normalised to 0/1).
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9f188.md.
// Style: natural -- plain call and bool normalisation
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9f188 0x28 \
//       xy-decomp/armcc-matching/matched/DllField/0x9f188.c

extern char *GetMgr(void);
extern int H(char *);
int Fn(void) {
    int r = 0;
    char *p = GetMgr();
    if (H(*(char **)(*(char **)(p + 12) + 100)) != 0) r = 1;
    return r;
}
