// DllField@0x9ad9c, size 0x28, 10/10 masked match.
// Script-command handler: stores u16 args[1] into word +8 of mgr->+12->+28->+56 and returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9ad9c.md.
// Style: trick(volatile) -- a is read through a volatile pointer only so armcc keeps the full-word ldr followed by the uxth narrowing (retail) instead of folding it into ldrh; verified: the plain-pointer form does not match.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9ad9c 0x28 \
//       xy-decomp/armcc-matching/matched/DllField/0x9ad9c.c

extern char *GetMgr(void);
int Fn(int self, volatile int *a) {
    unsigned short v = a[1];
    char *p = GetMgr();
    *(int *)(*(char **)(*(char **)(*(char **)(p + 12) + 28) + 56) + 8) = v;
    return 0;
}
