// DllField@0x5740, size 0x28, 10/10 masked match.
// Forwarding wrapper: calls an import veneer with (this, 0, 0, 0, 1, 0) --
// first four args in r0-r3, the trailing (1, 0) pushed on the stack.
// Same body at DllField 0x5740 and 0x5784.
//
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5740 0x28 \
//       xy-decomp/armcc-matching/matched/DllField/0x5740.c

extern void Imp(void *, int, int, int, int, int);

void Fn(void *this) {
    Imp(this, 0, 0, 0, 1, 0);
}
