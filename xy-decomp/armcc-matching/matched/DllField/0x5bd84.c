// DllField@0x5bd84, size 0x2c, masked match (all words).
// Calls E(*m + 0x1058, m[52]) with m = manager +12 -> +28; returns 0.
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5bd84 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x5bd84.c

extern char *GetMgr(void);
extern void E(int, int);
int Fn(void) {
    char *m = *(char **)(*(char **)(GetMgr() + 12) + 28);
    int base = *(int *)m;
    E(base + 0x1058, *(int *)(m + 52));
    return 0;
}
