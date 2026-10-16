// DllField@0x59c98, size 0x1c, 7/7 masked match.
// Calls H(G(a), b[1]) -- second arg is loaded from the second param before the first call -- and returns 0.
// b[1] is read into callee-saved r4 before the getter call.
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x59c98 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x59c98.c

extern void *G(void *);
extern void H(void *, int);
int Fn(void *a, int *b) {
    int v = b[1];
    H(G(a), v);
    return 0;
}
