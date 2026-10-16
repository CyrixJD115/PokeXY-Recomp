// DllField@0xa6ff0, size 0x14, 5/5 masked match.
// Setter: clears the byte flag at this+0xacd, then copies the field at
// this+0xac4 into this+0xab8 (load-ac4-first, then store-acd, then
// store-ab8 in that exact order -- matches retail's instruction order).
// Sibling of DllField@0xa714c (same two fields, hardcoded-1/swapped-copy
// variant right below it in .text).
//
// Style: trick(ordering-local) -- the copied word is loaded into a local before the flag store so the order is ldr, strb, str; the plain `flag = c; dst = src;` and `dst = src; flag = c;` forms both fail (tested)
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xa6ff0 0x14 \
//       xy-decomp/armcc-matching/matched/DllField/0xa6ff0.c

void Fn(void *this) {
    unsigned int v = *(unsigned int *)((unsigned char *)this + 0xac4);
    *((unsigned char *)this + 0xacd) = 0;
    *(unsigned int *)((unsigned char *)this + 0xab8) = v;
}
