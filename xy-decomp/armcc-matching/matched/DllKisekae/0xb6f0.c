/* DllKisekae@0xb6f0 - 3/3 match
 * Style: trick(redundant-local) -- redundant local (`int v = 1;`) only to force a register/store order
 * Byte-field setter: stores literal 1 at this+0x40a (1034), void return.
 * Retail's source explicitly materializes the 1 into r1 before the strb,
 * rather than letting armcc fold the immediate into the strb encoding.
 * Pass the literal directly as the store operand to keep the redundant
 * mov (armcc would otherwise fuse it into the strb at -O2 -Otime).
 * Verify: python3 tools/fingerprint_fn.py DllKisekae 0xb6f0 0xc matched/DllKisekae/0xb6f0.c
 */
void SetFlagTrue(unsigned char *this) {
    int v = 1;
    *(unsigned char *)(this + 0x40a) = (unsigned char)v;
}