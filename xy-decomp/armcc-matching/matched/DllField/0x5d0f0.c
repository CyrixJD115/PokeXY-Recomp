// DllField@0x5d0f0, size 0x40, masked match (all words).
// Script handler: b[0] split into hi/lo halfwords; if R(manager +12 -> +72 -> +360, hi, lo) succeeds calls Z(result, b[1]); returns 0.
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5d0f0 0x40 \
//       xy-decomp/armcc-matching/matched/DllField/0x5d0f0.c

extern char *GetMgr(void);
extern int R(int, int, int);
extern void Z(int, int);
int Fn(int a, int *b) {
    int w = b[0];
    int c = b[1];
    int hi = w >> 16;
    unsigned short lo = w;
    int r = R(*(int *)(*(char **)(*(char **)(GetMgr() + 12) + 72) + 360), hi, lo);
    if (r) Z(r, c);
    return 0;
}
