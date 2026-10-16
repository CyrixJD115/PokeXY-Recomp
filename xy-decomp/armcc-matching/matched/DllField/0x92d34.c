// DllField@0x92d34, size 0x38, 14/14 masked match.
// Script handler: r = Q(G(mgr->+12->+72), u16 args[1]); if (r) tail-call Tail(r).
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile-load) -- args pointer read through volatile so armcc keeps the full-word ldr + separate uxth/sxtb/and (retail) instead of folding into ldrh/ldrb
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x92d34 0x38 \
//       xy-decomp/armcc-matching/matched/DllField/0x92d34.c

extern char *GetMgr(void);
extern int G(int);
extern int Q(int, int);
extern void Tail(int);
void Fn(int a, volatile int *b) {
    unsigned short i = b[1];
    int r = Q(G(*(int *)(*(char **)(GetMgr() + 12) + 72)), i);
    if (r) Tail(r);
}
