// DllField@0x939cc, size 0x1c, 7/7 masked match.
// Tail-chains: returns F2(G(mgr->+12->+72)) (second call is a tail call).
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x939cc 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x939cc.c

extern char *GetMgr(void);
extern int G(int);
extern int T2(int);
int Fn(int self) {
    char *p=GetMgr();
    return T2(G(*(int*)(*(char**)(p+12)+72)));
}
