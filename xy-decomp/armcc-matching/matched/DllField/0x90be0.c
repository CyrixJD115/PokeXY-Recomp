// DllField@0x90be0, size 0x20, 8/8 masked match.
// Calls G(mgr->+12->+72) then T(); returns 0.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x90be0 0x20 \
//       xy-decomp/armcc-matching/matched/DllField/0x90be0.c

extern char *GetMgr(void);
extern int G(int);
extern void T(void);
int Fn(int self) {
    char *p=GetMgr();
    G(*(int*)(*(char**)(p+12)+72));
    T();
    return 0;
}
