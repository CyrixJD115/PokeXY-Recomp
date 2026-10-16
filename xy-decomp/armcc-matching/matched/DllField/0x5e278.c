// DllField@0x5e278, size 0x38, masked match (all words).
// Script handler: if helper(u16 b[1]) succeeds, calls a second helper with manager +12 -> +28 and the id; returns 0.
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: trick(volatile) -- b read through a volatile pointer only so armcc emits ldr+narrow (uxth/and) instead of ldrh/ldrb.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5e278 0x38 \
//       xy-decomp/armcc-matching/matched/DllField/0x5e278.c

extern char *GetMgr(void);
extern int F(int, int);
extern int F2(char *, int);
extern int P(int);
extern int G(int, int, int);
int Fn(int a, volatile int *b) {
    unsigned short x = b[1];
    if (P(x)) F2((*(char **)(*(char **)(GetMgr() + 12) + 28)), x);
    return 0;
}
