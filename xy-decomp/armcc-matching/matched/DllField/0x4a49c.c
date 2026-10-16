// DllField@0x4a49c, size 0x1c, 7/7 masked match.
// Calls an import veneer (this, 2, 34, this->byte@10) and returns 1.
// Call+return helper; byte field widened into r3, literal args 2/34.
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x4a49c 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x4a49c.c

extern void Imp(void *, int, int, int);
int Fn(void *this) {
    Imp(this, 2, 34, *(unsigned char *)((char *)this + 10));
    return 1;
}
