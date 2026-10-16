// DllField@0x5f504, size 0x1c, 7/7 masked match.
// Loads t=a->@68 first, calls G(a), then H(t->@144) and returns 0.
// The a->@68 load precedes the first call (callee-saved r4).
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5f504 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x5f504.c

extern void *G(void *);
extern void H(int);
int Fn(char *a) {
    char *t = *(char **)(a + 68);
    G(a);
    H(*(int *)(t + 144));
    return 0;
}
