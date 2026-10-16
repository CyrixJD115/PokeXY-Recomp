// DllField@0x543d4, size 0x1c, 7/7 masked match.
// Calls an import veneer (*arg0, 8, 1) and returns 3.
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x543d4 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x543d4.c

extern void Imp(void *, int, int);
int Fn(void **a) {
    Imp(*a, 8, 1);
    return 3;
}
