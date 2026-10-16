// DllField@0x98f30, size 0x28, 10/10 masked match.
// Calls helper on mgr->+12->+28->+52 then clears byte +1604 of it; returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x98f30.md.
// Style: natural -- plain pointer-chain loads, call, byte store
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x98f30 0x28 \
//       xy-decomp/armcc-matching/matched/DllField/0x98f30.c

extern char *GetMgr(void);
extern void Q(char *);
int Fn(void) {
    char *p = *(char **)(*(char **)(*(char **)(GetMgr() + 12) + 28) + 52);
    Q(p);
    *(char *)(p + 1604) = 0;
    return 0;
}
