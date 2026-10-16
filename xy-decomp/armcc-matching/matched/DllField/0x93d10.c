// DllField@0x93d10, size 0x2c, 11/11 masked match.
// Script handler: stores the low byte of args[1] into mgr->+12->+28->*->+0x1539c, returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile) -- b is read through a volatile pointer only so armcc loads the full arg word early (ldr r4,[r1,#4], before GetMgr()) and stores its low byte (strb) like retail; the plain-pointer form does not match (verified). The t local (arg read first) and the q pointer local are needed too (inline form verified not to match).
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x93d10 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x93d10.c

extern char *GetMgr(void);
int Fn(int a, volatile int *b) {
    int t = b[1];
    char *q = *(char **)(*(char **)(*(char **)(GetMgr() + 12) + 28)) + 0x1539c;
    *q = t;
    return 0;
}
