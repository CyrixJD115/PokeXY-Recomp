// DllField@0x54d84, size 0x38, 14/14 masked match.
// Call+return helper, sibling shape of DllField@0x5445c: calls an import
// veneer with (this->field0, 196), then a second import veneer with
// (this, this->byte@10, 1, 1, 12) -- 5th arg on the stack -- and returns
// 1 unconditionally. Same find_candidates.py size-rounding note as
// 0x5445c applies: reported size 0x40 includes two trailing one-word
// stubs (a `b <veneer>` forwarder and a bare `bx lr`) after this
// function's own `pop {r3,r4,r5,pc}`; real size is 0x38.
//
// Non-obvious bit: the two identical-valued arguments (both literal 1,
// landing in r2 and r3) must be written as the literal `1, 1` in the
// call, not through a named local variable holding 1 used twice --
// the named-variable form compiles with r2/r3's materialize-vs-copy
// roles swapped (armcc materializes into r2 first, copies into r3),
// while retail materializes into r3 first and copies into r2. The bare
// literal form reproduces retail's choice exactly on the first try.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x54d84 0x38 \
//       xy-decomp/armcc-matching/matched/DllField/0x54d84.c

extern void ImportB(void *, int);
extern void ImportA(void *, int, int, int, int);

int Fn(void *this) {
    ImportB(*(void **)this, 196);
    ImportA(this, *(unsigned char *)((char *)this + 10), 1, 1, 12);
    return 1;
}
