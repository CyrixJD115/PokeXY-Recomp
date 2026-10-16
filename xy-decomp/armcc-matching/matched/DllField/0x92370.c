// DllField@0x92370, size 0x40, 16/16 masked match.
// Float-field getter: as 0x92330 but returns (int)f[1] (offset +4).
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x92370 0x40 \
//       xy-decomp/armcc-matching/matched/DllField/0x92370.c

extern char *GetMgr(void);
extern void Assert(int,int,int);
int Fn(int self) {
    char *p=GetMgr();
    float *f=*(float**)(*(char**)(p+28)+148);
    if(f) return (int)f[1];
    Assert(0,0,0);
    return 0;
}
