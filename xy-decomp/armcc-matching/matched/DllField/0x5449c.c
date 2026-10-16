// DllField@0x5449c, size 0x1c, 7/7 masked match.
// Calls an import veneer (this, 2, 21, this->byte@10) and returns 1.
// Call+return helper; literal-only sibling of matched 0x4a49c.
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5449c 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x5449c.c

extern void Imp(void *, int, int, int);
int Fn(void *this) {
    Imp(this, 2, 21, *(unsigned char *)((char *)this + 10));
    return 1;
}
