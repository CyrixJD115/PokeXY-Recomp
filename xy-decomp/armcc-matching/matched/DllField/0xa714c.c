// DllField@0xa714c, size 0x14, 5/5 masked match.
// Setter: sets the byte flag at this+0xacd to 1, then copies the field at
// this+0xab8 into this+0xac4 (load-ab8-first, then store-acd, then
// store-ac4 in that exact order). Sibling of DllField@0xa6ff0, directly
// above it in .text -- same two fields, opposite copy direction and
// hardcoded flag value (0 vs 1).
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xa714c 0x14 \
//       xy-decomp/armcc-matching/matched/DllField/0xa714c.c

void Fn(void *this) {
    unsigned int v = *(unsigned int *)((unsigned char *)this + 0xab8);
    *((unsigned char *)this + 0xacd) = 1;
    *(unsigned int *)((unsigned char *)this + 0xac4) = v;
}
