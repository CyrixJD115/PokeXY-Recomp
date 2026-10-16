// DllField@0x948f0, size 0x2c, 11/11 masked match.
// Script handler: returns field +20 of the 28-byte table entry u16(args[1]) in mgr->+12->+28->+396.
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile-load) -- args pointer read through volatile so armcc keeps the full-word ldr + separate uxth/sxtb/and (retail) instead of folding into ldrh/ldrb
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x948f0 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x948f0.c

extern char *GetMgr(void);
struct E { int f[7]; };
int Fn(int a, volatile int *b) {
    unsigned short i = b[1];
    char *t = *(char **)(*(char **)(GetMgr() + 12) + 28);
    struct E *e = *(struct E **)(t + 396);
    return e[i].f[5];
}
