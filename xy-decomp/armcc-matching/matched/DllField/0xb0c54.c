// DllField@0xb0c54, size 0x20, 8/8 masked match.
// Clears a flag: if this->[+40] is non-null calls an import on it, then sets byte [+1077] = 0.
// Style: natural -- plain if/store
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xb0c54 0x20 \
//       xy-decomp/armcc-matching/matched/DllField/0xb0c54.c

extern void H(void *);
void Fn(char *t) {
    if (*(int *)(t + 40) != 0) H(*(void **)(t + 40));
    t[1077] = 0;
}
