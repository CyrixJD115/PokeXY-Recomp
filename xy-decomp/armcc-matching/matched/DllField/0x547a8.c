// DllField@0x547a8, size 0x18, 6/6 masked match.
// Calls an import veneer (arg1->field@16, arg2) and returns 1 (arg0 unused).
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x547a8 0x18 \
//       xy-decomp/armcc-matching/matched/DllField/0x547a8.c

extern void Imp(void *, int);
int Fn(void *a, int *b, int c) {
    Imp((void *)b[4], c);
    return 1;
}
