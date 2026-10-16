// DllField@0x93b30, size 0x30, 12/12 masked match.
// Script handler: H1(mgr->+44, u16 args[1]); H2(mgr); returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile) -- b is read through a volatile pointer only so armcc keeps the full-word ldr followed by the uxth narrowing (retail) instead of folding it into ldrh; verified: the plain-pointer form does not match.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x93b30 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0x93b30.c

extern char *GetMgr(void);
extern void H1(int, int);
extern void H2(char *);
int Fn(int a, volatile int *b) {
    unsigned short i = b[1];
    char *m = GetMgr();
    H1(*(int *)(m + 44), i);
    H2(m);
    return 0;
}
