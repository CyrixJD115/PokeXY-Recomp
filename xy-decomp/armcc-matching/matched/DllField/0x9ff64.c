// DllField@0x9ff64, size 0x24, 9/9 masked match.
// Script-command handler: calls helper on mgr->+20 with u16 args[1]; returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9ff64.md.
// Style: trick(volatile) -- args is volatile int* so armcc keeps ldr+uxth instead of ldrh
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9ff64 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0x9ff64.c

extern char *GetMgr(void);
extern void H(char *, int);
int Fn(int self, volatile int *a) {
    unsigned short v = a[1];
    char *p = GetMgr();
    H(*(char **)(p + 20), v);
    return 0;
}
