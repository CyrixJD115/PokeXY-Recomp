// DllField@0x5d45c, size 0x3c, masked match (all words).
// Script handler: b[1] split into hi/lo halfwords; if R(manager +12 -> +72 -> +360, hi, lo) succeeds calls a follow-up; returns 0.
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5d45c 0x3c \
//       xy-decomp/armcc-matching/matched/DllField/0x5d45c.c

extern char *GetMgr(void);
extern int F(int, int);
extern int K3(int, int, int);
extern int G(int, int, int);
extern int R(int, int, int);
extern void Z(void);
int Fn(int a, int *b) {
    int w = b[1];
    unsigned short lo = w;
    int hi = w >> 16;
    if (R(*(int *)(*(char **)(*(char **)(GetMgr() + 12) + 72) + 360), hi, lo)) Z();
    return 0;
}
