// DllField@0x5cfc8, size 0x2c, 11/11 masked match.
// Script handler: H(mgr->+12->+72->+240, (s16)args[1]); returns 0. Chain load made volatile to pin the late 'mov r1,r4' arg move.
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile-load) -- volatile arg read (full ldr + sxth) plus a volatile final chain load so armcc schedules mov r1,r4 after the chain like retail
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5cfc8 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x5cfc8.c

extern char *GetMgr(void);
extern void H(char *, int);
int Fn(int a, volatile int *b) {
    short t = b[1];
    char *p = *(char *volatile *)(*(char **)(*(char **)(GetMgr() + 12) + 72) + 240);
    H(p, t);
    return 0;
}
