// DllField@0xcf880, size 0x48, 18/18 masked match.
// State-handler step 1: if import A(this->[+112], 1) is non-zero calls import B(this->[+112], 1); clears byte [+260] and halfword [+0x102]; returns 1.
// Style: natural -- written plainly (state-handler step)
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xcf880 0x48 \
//       xy-decomp/armcc-matching/matched/DllField/0xcf880.c

extern int A(void *, int);
extern void B(void *, int);
int Fn(char *t) {
    int n = 1;
    if (A(*(void **)(t + 112), n)) B(*(void **)(t + 112), n);
    *(short *)(t + 0x102) = 0;
    t[260] = 0;
    return 1;
}
