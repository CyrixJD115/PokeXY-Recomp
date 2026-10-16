// DllField@0x98b84, size 0x38, 14/14 masked match.
// Script-command handler: sets byte args[1] on object mgr->+12->+64 via helper, then returns whether a second helper on it is nonzero.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x98b84.md.
// Style: trick(volatile-load) -- args is volatile int* so the narrowing and #255 happens after an ldr (not folded into ldrb); m/v locals only fix load order
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
