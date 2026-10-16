// DllField@0x91364, size 0x30, 12/12 masked match.
// Calls H1(mgr->+12->+72), H2(), H3(mgr->+28, 2); returns 0.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads, helper call; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x91364 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0x91364.c

extern char *GetMgr(void);
extern int H1(int);
extern void H2(void);
extern void H3(int, int);
int Fn(void) {
    char *p = GetMgr();
    H1(*(int *)(*(char **)(p + 12) + 72));
    H2();
    H3(*(int *)(p + 28), 2);
    return 0;
}
