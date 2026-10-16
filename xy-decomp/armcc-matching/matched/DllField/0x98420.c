// DllField@0x98420, size 0x2c, 11/11 masked match.
// Calls helper on mgr->+12->+28->*->+0x15fa8 with constant 4, returns the high halfword (result >> 16).
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x98420.md.
// Style: natural -- plain pointer-chain loads, call, shift
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x98420 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x98420.c

extern char *GetMgr(void);
extern unsigned Q(char *, int);
unsigned Fn(void) {
    char *p = GetMgr();
    return Q(*(char **)(*(char **)(*(char **)(p + 12) + 28)) + 0x15fa8, 4) >> 16;
}
