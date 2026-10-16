// DllField@0x97d9c, size 0x60, 24/24 masked match.
// Calls the assert stub (0,0,0), then if self->+72 is non-null calls Y(Z(self72)) else the assert stub again; then X(mgr->+28, 2); returns 1.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x97d9c 0x60 \
//       xy-decomp/armcc-matching/matched/DllField/0x97d9c.c

extern char *GetMgr(void);
extern void Assert(int,int,int);
extern int Z(int);
extern void Y(int);
extern void X(char*,int);
int Fn(int self) {
    char*p=GetMgr();
    int o=*(int*)(self+72);
    Assert(0,0,0);
    if(o) {
        Y(Z(o));
    }
    else Assert(0,0,0);
    X(*(char**)(p+28),2);
    return 1;
}
