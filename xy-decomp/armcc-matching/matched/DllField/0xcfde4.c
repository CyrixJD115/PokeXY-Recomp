// DllField@0xcfde4, size 0x48, 18/18 masked match.
// State-handler step 0: if import A(this->[+112], 0) is non-zero calls import B(this->[+112], 0); clears byte [+260] and halfword [+0x102]; returns 1.
// Style: trick(redundant-local) -- `int n = 0` named local only to keep the literal in a callee-saved register across the two imports; literal args at both call sites give a different sequence (length mismatch)
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xcfde4 0x48 \
//       xy-decomp/armcc-matching/matched/DllField/0xcfde4.c

extern int A(void *, int);
extern void B(void *, int);
int Fn(char *t) {
    int n = 0;
    if (A(*(void **)(t + 112), n)) B(*(void **)(t + 112), n);
    *(short *)(t + 0x102) = 0;
    t[260] = 0;
    return 1;
}
