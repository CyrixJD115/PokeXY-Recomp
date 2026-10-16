// DllField@0x37080, size 0x10, 4/4 masked match.
// Two-stage call chain (destructor-stage / forwarder shape): calls an import
// veneer with 'this' and tail-calls a second veneer with the first one's
// return value (r0 flows straight through, so no 'mov r4,r0'/'mov r0,r4').
// Same body at ~12 offsets in DllField; the following 2 words (a bare
// 'b <veneer>' and 'mov r0,#N; bx lr') belong to neighbouring stubs.
//
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x37080 0x10 \
//       xy-decomp/armcc-matching/matched/DllField/0x37080.c

extern void *A(void *);
extern void B(void *);

void Fn(void *this) {
    B(A(this));
}
