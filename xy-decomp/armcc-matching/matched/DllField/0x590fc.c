// DllField@0x590fc, size 0x30, masked match (all words).
// Script handler: u16 b[1] with (*(manager +12 -> +28)) + 0x3410 to a helper (tail call).
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: trick(volatile) -- b read through a volatile pointer only so armcc emits ldr+narrow (uxth/and) instead of ldrh/ldrb.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x590fc 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0x590fc.c

extern char *GetMgr(void);
extern int F(int, int);
extern int G(int, int, int);
int Fn(int a, volatile int *b) {
    unsigned short x = b[1];
    return F(*(int *)*(char **)(*(char **)(GetMgr() + 12) + 28) + 0x3410, x);
}
