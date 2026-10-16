// DllField@0x99604, size 0x34, 13/13 masked match.
// Script-command handler: calls helper on mgr->+12->+28->*->+0x129a8 with sign-extended s16 args[1]; returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x99604.md.
// Style: trick(volatile-load) -- args is volatile int* so armcc keeps ldr+sxth instead of ldrsh
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x99604 0x34 \
//       xy-decomp/armcc-matching/matched/DllField/0x99604.c

extern char *GetMgr(void);
extern void Q(char *, int);
int Fn(int self, volatile int *a) {
    short v = a[1];
    char *p = GetMgr();
    Q(*(char **)(*(char **)(*(char **)(p + 12) + 28)) + 0x129a8, v);
    return 0;
}
