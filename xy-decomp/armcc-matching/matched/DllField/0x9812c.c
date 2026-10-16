// DllField@0x9812c, size 0x2c, 11/11 masked match.
// Calls helper on mgr->+12->+28->*->+0x15fa8 with constant 3, returns the result as u16.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9812c.md.
// Style: natural -- plain pointer-chain loads and call
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9812c 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x9812c.c

extern char *GetMgr(void);
extern int Q(char *, int);
unsigned short Fn(void) {
    char *p = GetMgr();
    return Q(*(char **)(*(char **)(*(char **)(p + 12) + 28)) + 0x15fa8, 3);
}
