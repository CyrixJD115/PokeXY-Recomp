// DllField@0x98cec, size 0x28, 10/10 masked match.
// Script-command handler: stores u16 args[1] into word +4 of mgr->+12->+28->+56 and returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x98cec.md.
// Style: trick(volatile-load) -- args is volatile int* so armcc keeps ldr+uxth instead of ldrh
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x98cec 0x28 \
//       xy-decomp/armcc-matching/matched/DllField/0x98cec.c

extern char *GetMgr(void);
int Fn(int self, volatile int *a) {
    unsigned short v = a[1];
    char *p = GetMgr();
    *(int *)(*(char **)(*(char **)(*(char **)(p + 12) + 28) + 56) + 4) = v;
    return 0;
}
