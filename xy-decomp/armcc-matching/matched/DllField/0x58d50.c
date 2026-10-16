// DllField@0x58d50, size 0x2c, masked match (all words).
// Script handler: b[1] with (*(manager +12 -> +28)) + 0x3410 to a helper (tail call).
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x58d50 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x58d50.c

extern char *GetMgr(void);
extern int F(int, int);
extern int K3(int, int, int);
extern int G(int, int, int);
int Fn(int a, int *b) {
    return F(*(int *)*(char **)(*(char **)(GetMgr() + 12) + 28) + 0x3410, b[1]);
}
