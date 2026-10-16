// DllField@0x928ac, size 0x44, 17/17 masked match.
// Script handler: o = self->+68; GetMgr(); i = u8 args[1]; if (i >= 2) Err(0,0,0); returns byte at o->+144 + i*12 + 344. Volatile read placed AFTER the GetMgr() call (retail ldr [r5,#4] after bl).
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile-load) -- args pointer read through volatile so armcc keeps the full-word ldr + separate uxth/sxtb/and (retail) instead of folding into ldrh/ldrb
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x928ac 0x44 \
//       xy-decomp/armcc-matching/matched/DllField/0x928ac.c

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
    return *(unsigned char *)(tab + i * 12 + 344);
}
