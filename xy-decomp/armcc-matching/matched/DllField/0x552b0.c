// DllField@0x552b0, size 0x18, 6/6 masked match.
// Calls an import veneer (this, this->byte@10, 12) and returns 1.
// Call+return helper; siblings 0x552b0/0x552d0/0x552f0/0x55310 differ only in the literal (12/14/16/32).
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x552b0 0x18 \
//       xy-decomp/armcc-matching/matched/DllField/0x552b0.c

extern void Imp(void *, int, int);
int Fn(void *this) {
    Imp(this, *(unsigned char *)((char *)this + 10), 12);
    return 1;
}
