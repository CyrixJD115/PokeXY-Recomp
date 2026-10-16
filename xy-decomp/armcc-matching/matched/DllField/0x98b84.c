// DllField@0x98b84, size 0x38, 14/14 masked match.
// Script-command handler: sets byte args[1] on object mgr->+12->+64 via helper, then returns whether a second helper on it is nonzero.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x98b84.md.
// Style: trick(volatile) -- a is read through a volatile pointer only so armcc keeps the full-word ldr followed by the and #255 narrowing (retail) instead of folding it into ldrb; verified: the plain-pointer form does not match. The m/v/t locals only fix retail's load order (compact form verified not to match).
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x98b84 0x38 \
//       xy-decomp/armcc-matching/matched/DllField/0x98b84.c

extern char *GetMgr(void);
extern void Q(char *, int);
extern int R(char *);
int Fn(int self, volatile int *a) {
    char *m = *(char **)(GetMgr() + 12);
    int v = a[1] & 255;
    char *t = *(char **)(m + 64);
    Q(t, v);
    return R(t) != 0;
}
