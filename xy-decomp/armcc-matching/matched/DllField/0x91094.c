// DllField@0x91094, size 0x58, 22/22 masked match.
// Two-GetMgr sequence: m=mgr2->+12; v=A(m->+72->+344); w=B(m,v,0,1); C(mgr1,w); D(mgr1); E(mgr1); returns 0.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x91094 0x58 \
//       xy-decomp/armcc-matching/matched/DllField/0x91094.c

extern char *GetMgr(void);
extern int A(int);
extern int B(char*,int,int,int);
extern void C(char*,int);
extern void D(char*);
extern void E(char*);
int Fn(int self) {
    char*p=GetMgr();
    char*m=*(char**)(GetMgr()+12);
    int v=A(*(int*)(*(char**)(m+72)+344));
    int w=B(m,v,0,1);
    C(p,w);
    D(p);
    E(p);
    return 0;
}
