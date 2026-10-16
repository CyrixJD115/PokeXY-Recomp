// DllField@0x2b8d0, size 0x30, 12/12 masked match.
// If a check on the handle at this+24 passes, tail-calls a second veneer with
// (handle, 1, 1); otherwise returns.
//
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x2b8d0 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0x2b8d0.c

extern int Chk(int);
extern void Act(int, int, int);

void Fn(char *this) {
    if (Chk(*(int *)(this + 24)))
        Act(*(int *)(this + 24), 1, 1);
}
