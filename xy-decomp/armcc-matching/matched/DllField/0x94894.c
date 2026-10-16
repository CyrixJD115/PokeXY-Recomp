// DllField@0x94894, size 0x44, 17/17 masked match.
// Script handler: sibling of 0x928ac, returns word at o->+144 + i*12 + 340. Volatile read after GetMgr().
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile) -- b is read through a volatile pointer only so armcc keeps the full-word ldr followed by the and #255 narrowing (retail) instead of folding it into ldrb; verified: the plain-pointer form does not match. The t/tab/i locals only pin retail's load order (compact form verified not to match); the read sits after the GetMgr() call.
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
