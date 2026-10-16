// DllField@0x9ee48, size 0x18, 6/6 masked match.
// Returns the signed byte at +44 of the object mgr->+12->+100.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9ee48.md.
// Style: natural -- plain pointer-chain loads and ldrsb
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9ee48 0x18 \
//       xy-decomp/armcc-matching/matched/DllField/0x9ee48.c

extern char *GetMgr(void);
int Fn(void) {
    char *p = GetMgr();
    return *(signed char *)(*(char **)(*(char **)(p + 12) + 100) + 44);
}
