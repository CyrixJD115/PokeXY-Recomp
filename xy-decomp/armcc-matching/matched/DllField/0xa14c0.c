// DllField@0xa14c0, size 0x20, 8/8 masked match.
// Script-command handler: tail-calls helper on mgr->+24 with u16 args[1] and returns its result.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0xa14c0.md.
// Style: trick(volatile) -- args is volatile int* so armcc keeps ldr+uxth instead of ldrh
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xa14c0 0x20 \
//       xy-decomp/armcc-matching/matched/DllField/0xa14c0.c

extern char *GetMgr(void);
extern int H(char *, int);
int Fn(int self, volatile int *a) {
    unsigned short v = a[1];
    char *p = GetMgr();
    return H(*(char **)(p + 24), v);
}
