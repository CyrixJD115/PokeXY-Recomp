// DllField@0x9be50, size 0x34, 13/13 masked match.
// Script-command handler: calls helper on mgr->+12->+72->+408 with (args[1] != 0); returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9be50.md.
// Style: natural -- plain loads, bool normalisation, call
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9be50 0x34 \
//       xy-decomp/armcc-matching/matched/DllField/0x9be50.c

extern char *GetMgr(void);
extern void H(char *, int);
int Fn(int self, int *a) {
    int f = a[1] != 0;
    char *p = GetMgr();
    H(*(char **)(*(char **)(*(char **)(p + 12) + 72) + 408), f);
    return 0;
}
