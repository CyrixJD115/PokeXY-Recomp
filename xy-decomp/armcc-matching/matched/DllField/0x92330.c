// DllField@0x92330, size 0x40, 16/16 masked match.
// Float-field getter: if mgr->+28->+148 (float*) is non-null returns (int)f[0], else calls the assert/error stub (0x200 veneer) and returns 0.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x92330 0x40 \
//       xy-decomp/armcc-matching/matched/DllField/0x92330.c

extern char *GetMgr(void);
extern void Assert(int,int,int);
int Fn(int self) {
    char *p=GetMgr();
    float *f=*(float**)(*(char**)(p+28)+148);
    if(f) return (int)*f;
    Assert(0,0,0);
    return 0;
}
