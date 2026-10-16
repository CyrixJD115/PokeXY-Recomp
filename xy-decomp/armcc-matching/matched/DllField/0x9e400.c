// DllField@0x9e400, size 0x24, 9/9 masked match.
// Script-command handler: tail-calls helper on mgr->+12->+100 with u16 args[1] and returns its result.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9e400.md.
// Style: trick(volatile-load) -- args is volatile int* so armcc keeps ldr+uxth instead of ldrh
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9e400 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0x9e400.c

extern char *GetMgr(void);
extern int H(char *, int);
int Fn(int self, volatile int *a) {
    unsigned short v = a[1];
    char *p = GetMgr();
    return H(*(char **)(*(char **)(p + 12) + 100), v);
}
