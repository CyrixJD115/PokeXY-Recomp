// DllField@0xce874, size 0x50, 20/20 masked match.
// State-handler step 7: if import A(this->[+112], 7) is non-zero calls import B(this->[+112], 7); clears bytes [+278],[+279], halfword [+0x102] and byte [+260]; returns 1.
// Style: trick(redundant-local) -- `int n = 7` named local only to keep the literal in a callee-saved register across the two imports; literal args at both call sites give a different sequence (length mismatch)
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xce874 0x50 \
//       xy-decomp/armcc-matching/matched/DllField/0xce874.c

extern int A(void *, int);
extern void B(void *, int);
int Fn(char *t) {
    int n = 7;
    if (A(*(void **)(t + 112), n)) B(*(void **)(t + 112), n);
    t[278] = 0;
    t[279] = 0;
    *(short *)(t + 0x102) = 0;
    t[260] = 0;
    return 1;
}
