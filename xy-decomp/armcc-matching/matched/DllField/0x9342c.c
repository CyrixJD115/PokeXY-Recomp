// DllField@0x9342c, size 0x3c, 15/15 masked match.
// Bit-set test: i = args[1]; b = mgr->+12->+28->[0]+0x12e4 (bitset header); returns (R(b, b[1])[i>>3] >> (i&7)) & 1.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9342c 0x3c \
//       xy-decomp/armcc-matching/matched/DllField/0x9342c.c

extern char *GetMgr(void);
extern unsigned char *R(int*,int);
int Fn(int self,unsigned *a) {
    unsigned i=a[1];
    char *p=GetMgr();
    int *b=(int*)(*(char**)(*(char**)(*(char**)(p+12)+28))+0x12e4);
    return (R(b,b[1])[i>>3]>>(i&7))&1;
}
