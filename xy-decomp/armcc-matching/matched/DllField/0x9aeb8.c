// DllField@0x9aeb8, size 0x30, 12/12 masked match.
// Calls helper on mgr->+12->+28->*->+0x1aeec then returns the low 4 bits of the halfword at +4 of it.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9aeb8.md.
// Style: trick(volatile-load) -- halfword read through volatile so armcc keeps ldrh+and #15 (sibling of 0x9af9c)
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9aeb8 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0x9aeb8.c

extern char *GetMgr(void);
extern void Q(char *);
unsigned Fn(void) {
    char *p = *(char **)(*(char **)(*(char **)(GetMgr() + 12) + 28)) + 0x1aeec;
    Q(p);
    return *(volatile unsigned short *)(p + 4) & 15;
}
