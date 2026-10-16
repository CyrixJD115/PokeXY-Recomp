// DllField@0x94894, size 0x44, 17/17 masked match.
// Script handler: sibling of 0x928ac, returns word at o->+144 + i*12 + 340. Volatile read after GetMgr().
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile-load) -- args pointer read through volatile so armcc keeps the full-word ldr + separate uxth/sxtb/and (retail) instead of folding into ldrh/ldrb
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x94894 0x44 \
//       xy-decomp/armcc-matching/matched/DllField/0x94894.c

extern char *GetMgr(void);
extern void Err(int, int, int);
int Fn(char *self, volatile int *b) {
    char *o = *(char **)(self + 68);
    int t;
    char *tab;
    unsigned i;
    GetMgr();
    t = b[1];
    tab = *(char **)(o + 144);
    i = t & 255;
    if (i >= 2) Err(0, 0, 0);
    return *(int *)(tab + i * 12 + 340);
}
