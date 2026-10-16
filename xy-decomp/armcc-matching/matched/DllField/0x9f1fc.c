// DllField@0x9f1fc, size 0x30, 12/12 masked match.
// Script-command handler: calls helper on mgr->+12->+100 with (u16 args[1] != 0); returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9f1fc.md.
// Style: trick(volatile) -- args is volatile int* so armcc keeps ldr+uxth instead of ldrh
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9f1fc 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0x9f1fc.c

extern char *GetMgr(void);
extern void H(char *, int);
int Fn(int self, volatile int *a) {
    unsigned short v = a[1];
    char *p = GetMgr();
    H(*(char **)(*(char **)(p + 12) + 100), v != 0);
    return 0;
}
