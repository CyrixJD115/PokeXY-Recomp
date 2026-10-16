// DllField@0x5e068, size 0x30, masked match (all words).
// Script handler: b[1]&255 with (*(manager +12 -> +28)) + 0x1b814 to a helper (tail call).
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: trick(volatile) -- b is read through a volatile pointer only so armcc keeps the full-word ldr followed by the and #255 narrowing (retail) instead of folding it into ldrb; verified: the plain-pointer form does not match.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5e068 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0x5e068.c

extern char *GetMgr(void);
extern int F(int, int);
int Fn(int a, volatile int *b) {
    char *p = *(char **)(*(char **)(GetMgr() + 12) + 28);
    return F(*(int *)p + 0x1b814, b[1] & 255);
}
