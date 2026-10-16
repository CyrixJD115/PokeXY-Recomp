// DllField@0x90bc4, size 0x1c, 7/7 masked match.
// Getter: returns the byte at mgr->+12->+72->+344->+36.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x90bc4 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x90bc4.c

extern char *GetMgr(void);
unsigned char Fn(int self) {
    char *p=GetMgr();
    return *(*(char**)(*(char**)(*(char**)(p+12)+72)+344)+36);
}
