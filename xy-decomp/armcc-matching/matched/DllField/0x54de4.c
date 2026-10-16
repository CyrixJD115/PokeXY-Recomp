// DllField@0x54de4, size 0x1c, 7/7 masked match.
// Calls an import veneer (*arg0, 8, 0) and returns 3.
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x54de4 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x54de4.c

extern void Imp(void *, int, int);
int Fn(void **a) {
    Imp(*a, 8, 0);
    return 3;
}
