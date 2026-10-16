// DllField@0x9e7e4, size 0x28, 10/10 masked match.
// Script-command handler: calls helper on mgr->+12->+100 with u16 args[1]; returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9e7e4.md.
// Style: trick(volatile-load) -- args is volatile int* so armcc keeps ldr+uxth instead of ldrh
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9e7e4 0x28 \
//       xy-decomp/armcc-matching/matched/DllField/0x9e7e4.c

extern char *GetMgr(void);
extern void H(char *, int);
int Fn(int self, volatile int *a) {
    unsigned short v = a[1];
    char *p = GetMgr();
    H(*(char **)(*(char **)(p + 12) + 100), v);
    return 0;
}
