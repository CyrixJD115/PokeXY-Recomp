// DllField@0x5c490, size 0x2c, masked match (all words).
// Script handler: u16 arg b[1] and (*(manager +12 -> +28)) + 724 to a helper (tail call).
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: trick(volatile) -- b is read through a volatile pointer only so armcc emits ldr+narrow (uxth/and) instead of ldrh/ldrb.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5c490 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x5c490.c

extern char *GetMgr(void);
extern int F(int, int);
int Fn(int a, volatile int *b) {
    unsigned short x = b[1];
    return F(**(int **)(*(char **)(GetMgr() + 12) + 28) + 724, x);
}
