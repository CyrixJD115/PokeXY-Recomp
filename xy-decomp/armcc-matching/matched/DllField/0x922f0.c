// DllField@0x922f0, size 0x28, 10/10 masked match.
// Calls S3(mgr->+12->+28, args[1], args[2]) (ldm of the arg pair); returns 0.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x922f0 0x28 \
//       xy-decomp/armcc-matching/matched/DllField/0x922f0.c

extern char *GetMgr(void);
extern void S3(int,int,int);
int Fn(int self,int*a) {
    char *p=GetMgr();
    S3(*(int*)(*(char**)(p+12)+28),a[1],a[2]);
    return 0;
}
