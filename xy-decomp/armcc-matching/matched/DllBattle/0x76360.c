/*
 * DllBattle @ 0x76360, size 0x24 -- clears a 25-byte struct: six zero u32 stores (+0..+20) then a
 * zero byte at +24 (the count field read by the 0x5daf8/0x76384 getter). Masked match 9/9.
 * Family "zero-init struct": 0x76360 (this), 0xc5c18 (8 stores, 2 strb at +24/+25),
 * 0x71840 (u32+u16), 0xc9e58 (u32+u8), 0xc9e30 (u32,u32,u16).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x76360 0x24 xy-decomp/armcc-matching/matched/DllBattle/0x76360.c
 */
void fn(unsigned char *p) {
    *(unsigned *)p = 0; *(unsigned *)(p+4) = 0; *(unsigned *)(p+8) = 0;
    *(unsigned *)(p+12) = 0; *(unsigned *)(p+16) = 0; *(unsigned *)(p+20) = 0;
    p[24] = 0;
}
