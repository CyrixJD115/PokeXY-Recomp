// DllField@0x90b24, size 0x40, 16/16 masked match.
// Script handler: r = Q(G(mgr->+12->+72), u16 args[1]); returns r ? F(r) : 9.
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile) -- b is read through a volatile pointer only so armcc keeps the full-word ldr followed by the uxth narrowing (retail) instead of folding it into ldrh; verified: the plain-pointer form does not match.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x90b24 0x40 \
//       xy-decomp/armcc-matching/matched/DllField/0x90b24.c

extern char *GetMgr(void);
extern int G(int);
extern int Q(int, int);
extern int F(int);
int Fn(int a, volatile int *b) {
    unsigned short i = b[1];
    int r = Q(G(*(int *)(*(char **)(GetMgr() + 12) + 72)), i);
    int v = 9;
    if (r) v = F(r);
    return v;
}
