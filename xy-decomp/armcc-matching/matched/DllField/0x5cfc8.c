// DllField@0x5cfc8, size 0x2c, 11/11 masked match.
// Script handler: H(mgr->+12->+72->+240, (s16)args[1]); returns 0. Chain load made volatile to pin the late 'mov r1,r4' arg move.
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile) -- b is read through a volatile pointer only so armcc keeps the full-word ldr followed by the sxth narrowing (retail) instead of folding it into ldrsh; verified: the plain-pointer form does not match. A second volatile on the final chain load keeps `mov r1,r4` after the chain like retail (without it armcc hoists the move above the chain loads; verified).
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5cfc8 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x5cfc8.c

extern char *GetMgr(void);
extern void H(char *, int);
int Fn(int a, volatile int *b) {
    short t = b[1];
    char *p = *(char *volatile *)(*(char **)(*(char **)(GetMgr() + 12) + 72) + 240);
    H(p, t);
    return 0;
}
