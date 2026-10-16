// DllField@0x94c34, size 0x24, 9/9 masked match.
// Returns F2(F1(mgr->+12->+72->+344->+12)); the second call is a tail call.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x94c34 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0x94c34.c

extern char *GetMgr(void);
extern int F1(int);
extern int F2(int);
int Fn(int self) {
    char *p=GetMgr();
    return F2(F1(*(int*)(*(char**)(*(char**)(*(char**)(p+12)+72)+344)+12)));
}
