// DllField@0x92804, size 0x28, 10/10 masked match.
// Calls S2(mgr->+12->+72->+364, args[1]); returns 0.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x92804 0x28 \
//       xy-decomp/armcc-matching/matched/DllField/0x92804.c

extern char *GetMgr(void);
extern int S2(int,int);
int Fn(int self,int*a) {
    char *p=GetMgr();
    S2(*(int*)(*(char**)(*(char**)(p+12)+72)+364),a[1]);
    return 0;
}
