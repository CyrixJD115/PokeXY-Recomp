/* DllKisekae@0xb6f0 - 3/3 match
 * Byte-field setter: stores literal 1 at this+0x40a (1034), void return.
 * Style: natural
 * (Audit 2026-10-16: the earlier `int v = 1;` redundant local was unnecessary --
 * the plain literal store compiles to the same `mov r1,#1; strb` bytes.)
 *
 * Verify: python3 tools/fingerprint_fn.py DllKisekae 0xb6f0 0xc matched/DllKisekae/0xb6f0.c
 */
void SetFlagTrue(unsigned char *this) {
    *(unsigned char *)(this + 0x40a) = 1;
}
