// DllField@0x979dc, size 0x48, 18/18 masked match.
// Calls G(mgr->+12->+72->+244), then B(mgr); if G returned 0 calls the assert/error stub (0x528 veneer); returns 0.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x979dc 0x48 \
//       xy-decomp/armcc-matching/matched/DllField/0x979dc.c

extern char *GetMgr(void);
extern int G(int);
extern void Assert(int,int,int);
extern void B(char*);
int Fn(int self) {
    char *p=GetMgr();
    char *m=*(char**)(p+12);
    int r=G(*(int*)(*(char**)(m+72)+244));
    B(p);
    if(!r) Assert(0,0,0);
    return 0;
}
