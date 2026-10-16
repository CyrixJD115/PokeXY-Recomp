// DllField@0x945e4, size 0x38, 14/14 masked match.
// self-in-r4 sequence: v=F1(mgr->+12); A(mgr,v); B(mgr); C(mgr); returns 0.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x945e4 0x38 \
//       xy-decomp/armcc-matching/matched/DllField/0x945e4.c

extern char *GetMgr(void);
extern int F1(int);
extern void A(char*,int);
extern void B(char*);
extern void C(char*);
int Fn(int self) {
    char *p=GetMgr();
    int v=F1(*(int*)(p+12));
    A(p,v);
    B(p);
    C(p);
    return 0;
}
