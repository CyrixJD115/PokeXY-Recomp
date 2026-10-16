// DllField@0x9ea94, size 0x24, 9/9 masked match.
// Script-command handler: tail-calls helper on mgr->+12->+100 with u16 args[1] and returns its result.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9ea94.md.
// Style: trick(volatile) -- args is volatile int* so armcc keeps ldr+uxth instead of ldrh
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9ea94 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0x9ea94.c

extern char *GetMgr(void);
extern int H(char *, int);
int Fn(int self, volatile int *a) {
    unsigned short v = a[1];
    char *p = GetMgr();
    return H(*(char **)(*(char **)(p + 12) + 100), v);
}
