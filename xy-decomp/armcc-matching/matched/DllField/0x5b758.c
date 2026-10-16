// DllField@0x5b758, size 0x28, masked match (all words).
// Script handler: u8 arg b[1] forwarded with the manager's +12 -> +28 -> +20 object to a helper (tail call).
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: trick(volatile) -- b is read through a volatile pointer only so armcc emits ldr+narrow (uxth/and) instead of ldrh/ldrb.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5b758 0x28 \
//       xy-decomp/armcc-matching/matched/DllField/0x5b758.c

extern char *GetMgr(void);
extern int T(int, int);
int Fn(int a, volatile int *b) {
    unsigned char x = b[1];
    return T(*(int *)(*(char **)(*(char **)(GetMgr() + 12) + 28) + 20), x);
}
