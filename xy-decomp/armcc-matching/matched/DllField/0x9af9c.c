// DllField@0x9af9c, size 0x30, 12/12 masked match.
// Calls H(q) on the sub-object at mgr->+12->+28->[0] + 0x1aeec, then returns bits 4+ of its u16 field at +4.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads, helper call; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9af9c 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0x9af9c.c

extern char *GetMgr(void);
extern void H(char *);
int Fn(void) {
    char *p = GetMgr();
    char *b = *(char **)(*(char **)(*(char **)(p + 12) + 28));
    char *q = b + 0x1aeec;
    H(q);
    return (*(unsigned short *)(q + 4)) >> 4;
}
