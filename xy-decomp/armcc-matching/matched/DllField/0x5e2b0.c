// DllField@0x5e2b0, size 0x2c, masked match (all words).
// Calls H(m[400], *m + 0x13148) with m = manager +12 -> +28; returns 0.
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5e2b0 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x5e2b0.c

extern char *GetMgr(void);
extern void H(int, int);
int Fn(void) {
    char *m = *(char **)(*(char **)(GetMgr() + 12) + 28);
    H(*(int *)(m + 400), *(int *)m + 0x13148);
    return 0;
}
