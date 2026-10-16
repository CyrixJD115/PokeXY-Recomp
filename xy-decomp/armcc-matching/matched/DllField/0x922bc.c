// DllField@0x922bc, size 0x34, 13/13 masked match.
// Script handler: returns H(mgr->+12->+72->+404, u16 args[1], 0) != 0.
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile-load) -- args pointer read through volatile so armcc keeps the full-word ldr + separate uxth/sxtb/and (retail) instead of folding into ldrh/ldrb
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x922bc 0x34 \
//       xy-decomp/armcc-matching/matched/DllField/0x922bc.c

extern char *GetMgr(void);
extern int H(int, int, int);
int Fn(int a, volatile int *b) {
    unsigned short i = b[1];
    char *m = *(char **)(GetMgr() + 12);
    return H(*(int *)(*(char **)(m + 72) + 404), i, 0) != 0;
}
