// DllField@0x916a8, size 0x34, 13/13 masked match.
// Script handler: H(*(mgr->+12->+28) + 0x14cd0, u16 args[1]); returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile) -- b is read through a volatile pointer only so armcc keeps the full-word ldr followed by the uxth narrowing (retail) instead of folding it into ldrh; verified: the plain-pointer form does not match.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x916a8 0x34 \
//       xy-decomp/armcc-matching/matched/DllField/0x916a8.c

extern char *GetMgr(void);
extern void H(char *, int);
int Fn(int a, volatile int *b) {
    unsigned short i = b[1];
    char *p = *(char **)(*(char **)(GetMgr() + 12) + 28);
    H(*(char **)p + 0x14cd0, i);
    return 0;
}
