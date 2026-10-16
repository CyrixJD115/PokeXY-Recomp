// DllField@0x9515c, size 0x34, 13/13 masked match.
// Script handler: H(mgr->+12->+28->+36, args[1], args[2]&255); returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile) -- b is read through a volatile pointer only so armcc keeps the full-word ldr followed by the and #255 narrowing (retail) instead of folding it into ldrb; verified: the plain-pointer form does not match. The u/t/c locals (arg words read first, then narrowed) are needed too (inline form verified not to match).
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9515c 0x34 \
//       xy-decomp/armcc-matching/matched/DllField/0x9515c.c

extern char *GetMgr(void);
extern void H(int, int, int);
int Fn(int a, volatile int *b) {
    int u = b[2];
    int t = b[1];
    unsigned char c = u;
    H(*(int *)(*(char **)(*(char **)(GetMgr() + 12) + 28) + 36), t, c);
    return 0;
}
