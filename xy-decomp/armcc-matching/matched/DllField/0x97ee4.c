// DllField@0x97ee4, size 0x30, 12/12 masked match.
// Script handler: returns H(*(mgr->+12->+28) + 0x15fa8, args[1]&255) >> 16.
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile) -- b is read through a volatile pointer only so armcc keeps the full-word ldr followed by the and #255 narrowing (retail) instead of folding it into ldrb; verified: the plain-pointer form does not match.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x97ee4 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0x97ee4.c

extern char *GetMgr(void);
extern unsigned H(char *, int);
unsigned Fn(int a, volatile int *b) {
    int t = b[1];
    char *p = *(char **)(*(char **)(GetMgr() + 12) + 28);
    return H(*(char **)p + 0x15fa8, t & 255) >> 16;
}
