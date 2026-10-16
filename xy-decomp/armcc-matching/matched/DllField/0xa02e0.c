// DllField@0xa02e0, size 0x40, 16/16 masked match.
// Member function: asks helper H1(mgr->+24, u16 field this->+68->+148); if true returns (H2() == 0), else 1.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0xa02e0.md.
// Style: trick(volatile-load) -- the int field is read through volatile so armcc keeps ldr+uxth instead of ldrh; q/m locals only fix load order
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xa02e0 0x40 \
//       xy-decomp/armcc-matching/matched/DllField/0xa02e0.c

extern char *GetMgr(void);
extern int H1(char *, int);
extern int H2(void);
int Fn(char *self) {
    char *p = GetMgr();
    char *q = *(char **)(self + 68);
    char *m = *(char **)(p + 24);
    int x = *(volatile int *)(q + 148);
    if (H1(m, (unsigned short)x))
        return H2() == 0;
    return 1;
}
