// DllField@0x928ac, size 0x44, 17/17 masked match.
// Script handler: o = self->+68; GetMgr(); i = u8 args[1]; if (i >= 2) Err(0,0,0); returns byte at o->+144 + i*12 + 344. Volatile read placed AFTER the GetMgr() call (retail ldr [r5,#4] after bl).
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile) -- b is read through a volatile pointer only so armcc keeps the full-word ldr followed by the and #255 narrowing (retail) instead of folding it into ldrb; verified: the plain-pointer form does not match. The t/tab/i locals only pin retail's load order (compact form verified not to match); the read sits after the GetMgr() call.
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
