// DllField@0xd90cc, size 0x30, 12/12 masked match.
// 7-arg forwarding wrapper that reorders arguments into a 6-arg import:
// (a0..a6) -> Import(a0, a6, a5, a4, a3, a1); a2 is ignored. Result passed through.
// Style: natural -- plain argument reshuffle, no tricks needed.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xd90cc 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0xd90cc.c

extern int Import(int, int, int, int, int, int);

int Fn(int a0, int a1, int a2, int a3, int a4, int a5, int a6) {
    return Import(a0, a6, a5, a4, a3, a1);
}
