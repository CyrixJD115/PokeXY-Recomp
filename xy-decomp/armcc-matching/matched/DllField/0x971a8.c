// DllField@0x971a8, size 0x3c, 15/15 masked match.
// Returns (int)F(mgr->+12->+72->+344->+12) / 18 (float getter converted and divided by 18 via smull magic).
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x971a8 0x3c \
//       xy-decomp/armcc-matching/matched/DllField/0x971a8.c

extern char *GetMgr(void);
extern float F(int);
int Fn(int self) {
    char *p=GetMgr();
    return (int)F(*(int*)(*(char**)(*(char**)(*(char**)(p+12)+72)+344)+12))/18;
}
