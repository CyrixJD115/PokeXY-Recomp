// DllField@0x2aee0, size 0x1c, 7/7 masked match.
// Calls an import veneer with the constant 8, then clears the byte at
// this+32 and returns that zero (char return of the assignment expression;
// retail reuses the single 'mov r0,#0' for both the store and the return).
// Same body at DllField 0x2aa40, 0x2aee0, 0x343d0.
//
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x2aee0 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x2aee0.c

extern void Imp(int);

char Fn(char *this) {
    Imp(8);
    return this[32] = 0;
}
