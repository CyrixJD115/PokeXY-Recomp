// DllField@0xc74c4, size 0x28, 10/10 masked match.
// Calls an import check on `this`; if it reports true, clears the two words
// at +164/+168 and returns the byte flag at +188; otherwise returns 0.
// Byte-identical sibling at 0xcac98 (same C, see matched/DllField/0xcac98.c).
// Style: natural -- early-return-0 guard, then clear fields and read the flag.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xc74c4 0x28 \
//       xy-decomp/armcc-matching/matched/DllField/0xc74c4.c

extern int Check(void *);

int Fn(char *this) {
    if (Check(this) == 0) return 0;
    *(int *)(this + 164) = 0;
    *(int *)(this + 168) = 0;
    return *(unsigned char *)(this + 188);
}
