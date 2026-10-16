// DllField@0x97a24, size 0x28, 10/10 masked match.
// Returns byte [6] of the struct returned by R6(mgr->+12->+28->+40, 3, 0).
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x97a24 0x28 \
//       xy-decomp/armcc-matching/matched/DllField/0x97a24.c

extern char *GetMgr(void);
extern char *R6(int,int,int);
unsigned char Fn(int self) {
    char *p=GetMgr();
    return R6(*(int*)(*(char**)(*(char**)(p+12)+28)+40),3,0)[6];
}
