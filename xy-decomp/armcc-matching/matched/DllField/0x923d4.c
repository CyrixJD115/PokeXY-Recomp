// DllField@0x923d4, size 0x1c, 7/7 masked match.
// Getter: loads self->+68, fetches the manager (result unused), returns the signed byte at (self68->+144)+0x100+76.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x923d4 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x923d4.c

extern char *GetMgr(void);
signed char Fn(int self) {
    char *o=*(char**)((char*)self+68);
    GetMgr();
    return *(signed char*)(*(char**)(o+144)+0x100+76);
}
