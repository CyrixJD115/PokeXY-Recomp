// DllField@0xb8724, size 0x18, 6/6 masked match.
// Getter: returns 105 if an import call returns 0, else 0.
// Style: natural -- plain ternary
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xb8724 0x18 \
//       xy-decomp/armcc-matching/matched/DllField/0xb8724.c

extern int G(void);
int Fn(void) { return G() == 0 ? 105 : 0; }
