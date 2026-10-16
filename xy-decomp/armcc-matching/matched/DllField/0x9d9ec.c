// DllField@0x9d9ec, size 0x34, 13/13 masked match.
// Calls F on mgr->+12->+24 and passes the result to a 5-arg helper (33, 0, 0, 0); returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9d9ec.md.
// Style: natural -- plain call chain with 5th arg on the stack
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9d9ec 0x34 \
//       xy-decomp/armcc-matching/matched/DllField/0x9d9ec.c

extern char *GetMgr(void);
extern char *F(char *);
extern void H(char *, int, int, int, int);
int Fn(void) {
    H(F(*(char **)(*(char **)(GetMgr() + 12) + 24)), 33, 0, 0, 0);
    return 0;
}
