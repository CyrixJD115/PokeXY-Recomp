// DllField@0x543b4, size 0x18, 6/6 masked match.
// Calls an import veneer (*arg0, 16) and returns 3.
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x543b4 0x18 \
//       xy-decomp/armcc-matching/matched/DllField/0x543b4.c

extern void Imp(void *, int);
int Fn(void **a) {
    Imp(*a, 16);
    return 3;
}
