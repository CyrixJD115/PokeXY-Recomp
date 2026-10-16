// DllField@0x5edcc, size 0x3c, masked match (all words).
// Script handler: u16 id checked via Q(H(manager +12 -> +72), id); returns 1 if not found else tail-calls Tail(r).
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: trick(volatile) -- b is read through a volatile pointer only so armcc keeps the full-word ldr followed by the uxth narrowing (retail) instead of folding it into ldrh; verified: the plain-pointer form does not match.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5edcc 0x3c \
//       xy-decomp/armcc-matching/matched/DllField/0x5edcc.c

extern char *GetMgr(void);
extern int H(int);
extern int Q(int, int);
extern int Tail(int);
int Fn(int a, volatile int *b) {
    unsigned short x = b[1];
    int r = Q(H(*(int *)(*(char **)(GetMgr() + 12) + 72)), x);
    if (r == 0) return 1;
    return Tail(r);
}
