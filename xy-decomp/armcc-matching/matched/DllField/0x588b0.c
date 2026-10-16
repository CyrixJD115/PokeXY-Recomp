// DllField@0x588b0, size 0x30, 12/12 masked match.
// Script handler: H(mgr->+12->+72->+240->+2740 + 420, (u16)args[1]); returns 0. Two volatile chain loads pin the late uxth.
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile) -- volatile arg read (full ldr; uxth at use) AND volatile last two chain loads; dropping the chain volatiles fails (verified), so armcc needs them to keep the late uxth after the chain like retail
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x588b0 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0x588b0.c

extern char *GetMgr(void);
extern void H(char *, int);
int Fn(int a, volatile int *b) {
    int t = b[1];
    char *p = *(char *volatile *)(*(char **)(*(char **)(GetMgr() + 12) + 72) + 240);
    char *q = *(char *volatile *)(p + 2740);
    H(q + 420, (unsigned short)t);
    return 0;
}
