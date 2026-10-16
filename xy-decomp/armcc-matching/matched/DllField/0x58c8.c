// DllField@0x58c8, size 0xc, 3/3 masked match.
// Trivial hardcoded-true byte-field setter: ignores any second argument,
// unconditionally sets the byte field at this+0x1c (28) to 1, void return.
// Sits right after DllField's large import-veneer table (0x180-0x5767) and
// a handful of small veneer-calling wrappers/float helpers (0x5740-0x58c4);
// the next function starts immediately after at 0x58d4.
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x58c8 0xc \
//       xy-decomp/armcc-matching/matched/DllField/0x58c8.c

void Fn(void *this) {
    *((unsigned char *)this + 28) = 1;
}
