// DllField@0x132b8, size 0x1c, 7/7 masked match.
// Predicate: passes this->field@0x24 to a callee and returns its logical negation (1 if result is 0).
// Style: natural -- if (r) return 0; return 1; form
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x132b8 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x132b8.c

extern int V(int);
int Fn(int *this) { if (V(this[9])) return 0; return 1; }
