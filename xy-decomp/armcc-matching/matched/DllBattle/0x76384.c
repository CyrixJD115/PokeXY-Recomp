/*
 * DllBattle @ 0x76384, size 0x14 -- bounds-checked u32 array getter: returns ((u32*)this)[i] when
 * i < this->count (u8 at +24), else 0. Masked match 5/5 (ldrhi/movls conditional execution).
 * Same bytes/C as sibling 0x5daf8.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x76384 0x14 xy-decomp/armcc-matching/matched/DllBattle/0x76384.c
 */
unsigned fn(unsigned char *p, unsigned i) {
    if (i < p[24]) return ((unsigned *)p)[i];
    return 0;
}
