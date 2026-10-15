// DllField@0x54dc4, size 0x18, 6/6 masked match.
// Call+return helper: calls an import veneer with (this->field0, 16),
// then returns the hardcoded constant 3. Found by scanning right after
// the matched 0x54d84/0x5445c pair in the same stub-cluster region of
// .text; the next function (0x54de4, same shape but with a different
// veneer and argument (field0, 0, 8)) looks like an equally easy
// follow-up for a future agent.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x54dc4 0x18 \
//       xy-decomp/armcc-matching/matched/DllField/0x54dc4.c

extern void ImportC(void *, int);

int Fn(void *this) {
    ImportC(*(void **)this, 16);
    return 3;
}
