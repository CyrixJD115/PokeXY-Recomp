// DllField@0x94d18, size 0x30, 12/12 masked match.
// Script handler: H(*(mgr->+12->+28) + 0x14cd0, (s8)args[1]); returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile-load) -- args pointer read through volatile so armcc keeps the full-word ldr + separate uxth/sxtb/and (retail) instead of folding into ldrh/ldrb
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x94d18 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0x94d18.c

extern char *GetMgr(void);
extern void H(char *, int);
int Fn(int a, volatile int *b) {
    int t = b[1];
    char *p = *(char **)(*(char **)(GetMgr() + 12) + 28);
    H(*(char **)p + 0x14cd0, (signed char)t);
    return 0;
}
