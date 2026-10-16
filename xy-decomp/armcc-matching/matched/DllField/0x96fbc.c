// DllField@0x96fbc, size 0x2c, 11/11 masked match.
// Returns R4(mgr->+12->+28->[0] + 0x15fa8, 3) >> 16 (high half of a packed result).
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x96fbc 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x96fbc.c

extern char *GetMgr(void);
extern unsigned R4(char*,int); unsigned Fn(int self) {
    char *p=GetMgr();
    return R4(*(char**)(*(char**)(*(char**)(p+12)+28))+0x15fa8,3)>>16;
}
