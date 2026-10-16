// DllField@0x5db64, size 0x34, masked match (all words).
// Script handler: b[1] != 0 flag with manager +12 -> +28 -> +380 object to a helper; returns 0.
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5db64 0x34 \
//       xy-decomp/armcc-matching/matched/DllField/0x5db64.c

extern char *GetMgr(void);
extern int F(int, int);
extern int G(int, int, int);
int Fn(int a, int *b) {
    int x = b[1] != 0;
    F(*(int *)(*(char **)(*(char **)(GetMgr() + 12) + 28) + 380), x);
    return 0;
}
