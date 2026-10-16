// DllField@0xcff28, size 0x60, 24/24 masked match.
// State-handler step: if byte [+276]==0 calls P(this, -float[+272], 0); then Q(this, byte[+277]); R(this, 0.0f, 0.0f); clears halfword [+0x102] and byte [+260]; returns 1.
// Style: trick(redundant-local) -- p = t+0x100 named local pins retail's early add r5,r0,#256 and the strh [r5,#2]
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xcff28 0x60 \
//       xy-decomp/armcc-matching/matched/DllField/0xcff28.c

extern void P(void *, float, int);
extern void Q(void *, int);
extern void R(void *, float, float);
int Fn(char *t) {
    char *p = t + 0x100;
    if (t[276] == 0) P(t, -*(float *)(t + 272), 0);
    Q(t, t[277]);
    R(t, 0.0f, 0.0f);
    *(short *)(p + 2) = 0;
    t[260] = 0;
    return 1;
}
