// DllField@0x9d534, size 0x30, 12/12 masked match.
// Script-command handler: H1(mgr->+12->+28->+20, u16 args[1]) then tail-calls H2(result, 8).
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9d534.md.
// Style: trick(volatile) -- a is read through a volatile pointer only so armcc keeps the full-word ldr followed by the uxth narrowing (retail) instead of folding it into ldrh; verified: the plain-pointer form does not match.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9d534 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0x9d534.c

extern char *GetMgr(void);
extern int H1(char *, int);
extern int H2(int, int);
int Fn(int self, volatile int *a) {
    unsigned short v = a[1];
    char *p = GetMgr();
    return H2(H1(*(char **)(*(char **)(*(char **)(p + 12) + 28) + 20), v), 8);
}
