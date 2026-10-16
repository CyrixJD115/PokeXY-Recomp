// DllField@0x9484c, size 0x48, 18/18 masked match.
// Script handler: sibling of 0x928ac, returns u16 at o->+144 + i*12 + 336 (the local unsigned short *p stops armcc folding the offset into ldrh imm: retail add #336; ldrh [r0]).
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile-load) -- volatile arg read after GetMgr() keeps retail full-word ldr + and #255; also a local halfword pointer p so armcc keeps add #336 + ldrh [r0]
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9484c 0x48 \
//       xy-decomp/armcc-matching/matched/DllField/0x9484c.c

extern char *GetMgr(void);
extern void Err(int, int, int);
int Fn(char *self, volatile int *b) {
    char *o = *(char **)(self + 68);
    int t;
    char *tab;
    unsigned i;
    unsigned short *p;
    GetMgr();
    t = b[1];
    tab = *(char **)(o + 144);
    i = t & 255;
    if (i >= 2) Err(0, 0, 0);
    p = (unsigned short *)(tab + i * 12 + 336);
    return *p;
}
