// DllField@0x59b04, size 0x24, masked match (all words).
// Same as 0x593cc with a different tail helper.
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: trick(volatile) -- b is read through a volatile pointer only so armcc emits ldr+narrow (uxth/and) instead of ldrh/ldrb.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x59b04 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0x59b04.c

extern char *GetMgr(void);
extern int T(int, int);
int Fn(int a, volatile int *b) {
    unsigned short x = b[1];
    return T(*(int *)(*(char **)(GetMgr() + 16) + 4), x);
}
