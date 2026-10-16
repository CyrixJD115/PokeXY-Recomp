// DllField@0x5a6b4, size 0x4c, masked match (all words).
// Same check; on success returns Tail(r), on failure calls Other(0,0,0) and returns 0.
// Reads the u16 at b[1] (retail does a full-word ldr + uxth), asks the manager
// (0x16c0 -> +12 -> +72) through helper 0x865cc, then tests Q(helper_result, id);
// on nonzero tail-calls a follow-up with Q's result.
// Style: trick(volatile) -- b is read through a volatile int pointer only so armcc emits ldr+uxth instead of ldrh.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5a6b4 0x4c \
//       xy-decomp/armcc-matching/matched/DllField/0x5a6b4.c

extern char *GetMgr(void);
extern int H(int);
extern int Q(int, int);
extern int Tail(int);
extern int Other(int, int, int);
int Fn(int a, volatile int *b) {
    unsigned short x = b[1];
    int r = Q(H(*(int *)(*(char **)(GetMgr() + 12) + 72)), x);
    if (r) return Tail(r);
    Other(0, 0, 0);
    return 0;
}
