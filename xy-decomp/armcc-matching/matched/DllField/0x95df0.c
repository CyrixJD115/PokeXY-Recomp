// DllField@0x95df0, size 0x5c, 23/23 masked match.
// Script handler: if Q(...) succeeds, s = G2(); s ? Z(s,0) : Err(0,0,0); returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile-load) -- args pointer read through volatile so armcc keeps the full-word ldr + separate uxth/sxtb/and (retail) instead of folding into ldrh/ldrb
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x95df0 0x5c \
//       xy-decomp/armcc-matching/matched/DllField/0x95df0.c

extern char *GetMgr(void);
extern int G(int);
extern int Q(int, int);
extern int G2();
extern void Z(int, int);
extern void Err(int, int, int);
int Fn(int a, volatile int *b) {
    unsigned short i = b[1];
    int r = Q(G(*(int *)(*(char **)(GetMgr() + 12) + 72)), i);
    if (r) {
        int s = G2();
        if (s) Z(s, 0);
        else Err(0, 0, 0);
    }
    return 0;
}
