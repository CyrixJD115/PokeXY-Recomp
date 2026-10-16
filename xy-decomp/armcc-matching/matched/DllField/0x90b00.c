// DllField@0x90b00, size 0x24, 9/9 masked match.
// Reads args[1] and calls S(mgr->+44, 3, args[1]) (different callee than 0x90adc); returns 0.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x90b00 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0x90b00.c

extern char *GetMgr(void);
extern void S(int,int,int);
int Fn(int self,int*a) {
    int v=a[1];
    char *p=GetMgr();
    S(*(int*)(p+44),3,v);
    return 0;
}
