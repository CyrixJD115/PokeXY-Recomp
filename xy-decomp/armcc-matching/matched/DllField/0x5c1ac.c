// DllField@0x5c1ac, size 0x38, masked match (all words).
// Script handler: b[1] != 0 flag with (*(manager +12 -> +28)) + 0x14444 to a helper; returns 0.
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5c1ac 0x38 \
//       xy-decomp/armcc-matching/matched/DllField/0x5c1ac.c

extern char *GetMgr(void);
extern int F(int, int);
extern int G(int, int, int);
int Fn(int a, int *b) {
    F(*(int *)*(char **)(*(char **)(GetMgr() + 12) + 28) + 0x14444, b[1] != 0);
    return 0;
}
