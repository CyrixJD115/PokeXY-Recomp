// DllField@0x91590, size 0x20, 8/8 masked match.
// Boolean getter: loads self->+68, fetches the manager (unused), returns (self68->+144)->+16 != 0.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x91590 0x20 \
//       xy-decomp/armcc-matching/matched/DllField/0x91590.c

extern char *GetMgr(void);
int Fn(int self) {
    char *o=*(char**)((char*)self+68);
    GetMgr();
    return *(int*)(*(char**)(o+144)+16)!=0;
}
