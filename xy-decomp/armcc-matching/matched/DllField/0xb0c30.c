// DllField@0xb0c30, size 0x24, 9/9 masked match.
// Sets a flag: if this->[+40] is non-null calls an import on it with 1, then sets byte [+1077] = 1.
// Style: natural -- plain if/store
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xb0c30 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0xb0c30.c

extern void H(void *, int);
void Fn(char *t) {
    if (*(int *)(t + 40) != 0) H(*(void **)(t + 40), 1);
    t[1077] = 1;
}
