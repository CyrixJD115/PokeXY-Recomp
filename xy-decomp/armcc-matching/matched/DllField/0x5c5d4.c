// DllField@0x5c5d4, size 0x1c, masked match (all words).
// Script handler: u8 arg b[1] forwarded with the manager pointer to a helper (tail call).
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: trick(volatile) -- b is read through a volatile pointer only so armcc keeps the full-word ldr followed by the and #255 narrowing (retail) instead of folding it into ldrb; verified: the plain-pointer form does not match.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5c5d4 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x5c5d4.c

extern char *GetMgr(void);
extern int T(int, int);
int Fn(int a, volatile int *b) {
    unsigned char x = b[1];
    return T((int)GetMgr(), x);
}
