// DllField@0x5d698, size 0x38, masked match (all words).
// Id check: if Q(manager item, id) succeeds, tail-call a follow-up.
// Reads the u16 at b[1] (retail does a full-word ldr + uxth), asks the manager
// (0x16c0 -> +12 -> +72) through helper 0x865cc, then tests Q(helper_result, id);
// on nonzero tail-calls a follow-up with Q's result.
// Style: trick(volatile) -- b is read through a volatile pointer only so armcc keeps the full-word ldr followed by the uxth narrowing (retail) instead of folding it into ldrh; verified: the plain-pointer form does not match.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5d698 0x38 \
//       xy-decomp/armcc-matching/matched/DllField/0x5d698.c

extern char *GetMgr(void);
extern int H(int);
extern int Q(int, int);
extern void Tail(int);
void Fn(int a, volatile int *b) {
    unsigned short x = b[1];
    int r = Q(H(*(int *)(*(char **)(GetMgr() + 12) + 72)), x);
    if (r) Tail(r);
}
