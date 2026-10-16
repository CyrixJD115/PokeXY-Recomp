// DllField@0x91330, size 0x34, 13/13 masked match.
// Script handler: H1(mgr->+12, u16 args[1], 0); H2(mgr); returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile-load) -- args pointer read through volatile so armcc keeps the full-word ldr + separate uxth/sxtb/and (retail) instead of folding into ldrh/ldrb
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x91330 0x34 \
//       xy-decomp/armcc-matching/matched/DllField/0x91330.c

extern char *GetMgr(void);
extern void H1(int, int, int);
extern void H2(char *);
int Fn(int a, volatile int *b) {
    unsigned short i = b[1];
    char *m = GetMgr();
    H1(*(int *)(m + 12), i, 0);
    H2(m);
    return 0;
}
