// DllField@0x2aee0, size 0x1c, 7/7 masked match.
// Calls an import veneer with the constant 8, then clears the byte at
// this+32 and returns that zero (char return of the assignment expression;
// retail reuses the single 'mov r0,#0' for both the store and the return).
// Same body at DllField 0x2aa40, 0x2aee0, 0x343d0.
//
// Style: trick(assign-as-return) -- the `char` return value exists only so `return this[32] = 0;` lets armcc reuse the one
// `mov r0,#0` for store and return; a void body or `this[32]=0; return 0;` both give a spare `mov r1,#0` (verified).
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x2aee0 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x2aee0.c

extern void Imp(int);

char Fn(char *this) {
    Imp(8);
    return this[32] = 0;
}
