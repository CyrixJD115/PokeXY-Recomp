// DllField@0xa0c34, size 0x40, 16/16 masked match.
// Script-command handler: looks up an entry F(mgr->+16, u16 args[1]); returns its word +8, or (after an error call E(0,0,0)) 0 if missing.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0xa0c34.md.
// Style: trick(volatile) -- args is volatile int* so armcc keeps ldr+uxth instead of ldrh
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xa0c34 0x40 \
//       xy-decomp/armcc-matching/matched/DllField/0xa0c34.c

extern char *GetMgr(void);
extern char *F(char *, int);
extern void E(int, int, int);
int Fn(int self, volatile int *a) {
    unsigned short v = a[1];
    char *p = GetMgr();
    char *r = F(*(char **)(p + 16), v);
    if (r) return *(int *)(r + 8);
    E(0, 0, 0);
    return 0;
}
