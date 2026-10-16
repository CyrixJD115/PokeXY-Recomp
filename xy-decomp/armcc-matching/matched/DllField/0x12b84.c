// DllField@0x12b84, size 0x34 (0x30 code + 1 literal pool word), 13/13 masked match.
// State step: calls an import veneer with (0x6002f, 0, -1, 0) (a constant id
// from the literal pool, not a relocated address), sets byte +89 of the
// object at this+32 to 1, and sets byte this+37 to 1.
//
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x12b84 0x34 \
//       xy-decomp/armcc-matching/matched/DllField/0x12b84.c

extern void Imp(int, int, int, int);

void Fn(char *this) {
    Imp(0x6002f, 0, -1, 0);
    *(char *)(*(char **)(this + 32) + 89) = 1;
    this[37] = 1;
}
