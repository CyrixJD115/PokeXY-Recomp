// DllField@0x95644, size 0x24, 9/9 masked match.
// Calls S2(mgr->+12->+72->+356, 1); returns 0.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x95644 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0x95644.c

extern char *GetMgr(void);
extern int S2(int,int);
int Fn(int self,int*a) {
    char *p=GetMgr();
    S2(*(int*)(*(char**)(*(char**)(p+12)+72)+356),1);
    return 0;
}
