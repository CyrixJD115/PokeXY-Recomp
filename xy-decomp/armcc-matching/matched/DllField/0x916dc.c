// DllField@0x916dc, size 0x30, 12/12 masked match.
// If mgr->+28 has a non-null pointer at +148, calls H() and clears the word at (mgr->+28)+124+24; returns 0.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads, helper call; q is the address of an embedded sub-struct at +124 (retail tests/clears it as q+24).
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x916dc 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0x916dc.c

extern char *GetMgr(void);
extern void H(void);
int Fn(void) {
    char *p = GetMgr();
    char *m = *(char **)(p + 28);
    char *q = m + 124;
    if (*(int *)(m + 148) != 0) {
        H();
        *(int *)(q + 24) = 0;
    }
    return 0;
}
