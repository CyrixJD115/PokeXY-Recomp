// DllField@0xe4588, size 0x20, 8/8 masked match.
// Loads this->[+8]->[+20], passes it to an import and returns whether the
// (unsigned) result is < 6. The retail movcc/movcs pair means the compare is
// unsigned -- a signed `< 6` compiles to movlt/movge instead.
// Style: natural -- unsigned result compared against a constant.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xe4588 0x20 \
//       xy-decomp/armcc-matching/matched/DllField/0xe4588.c

extern unsigned ImportA(int);

int Fn(int *this) {
    return ImportA(*(int *)(*(int *)((char *)this + 8) + 20)) < 6u;
}
