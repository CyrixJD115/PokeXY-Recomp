// DllField@0x5ed88, size 0x20, masked match (all words).
// Chain of three helper calls on the manager's +12 object: C(B(A(m), 0)) (last is a tail call).
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5ed88 0x20 \
//       xy-decomp/armcc-matching/matched/DllField/0x5ed88.c

extern char *GetMgr(void);
extern int A(int);
extern int B(int, int);
extern int C(int);
int Fn(void) {
    return C(B(A(*(int *)(GetMgr() + 12)), 0));
}
