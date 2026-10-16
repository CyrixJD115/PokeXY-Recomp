/*
 * DllWazaOboe @ 0x82c, size 0xc, masked match 3/3.
 * Field setter + return 1: sets a byte flag at offset +12 of the object
 * passed in the 3rd argument register (r2) to 1, ignores `this` (r0) and
 * the 2nd argument (r1), returns 1. Sits right after the 0x7ec-0x824 leaf
 * stub run, right before a real function at 0x838 (push {r4-r8,lr}; vpush
 * {d8} — a constructor-shaped function, not attempted).
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllWazaOboe \
 *       0x82c 0xc xy-decomp/armcc-matching/matched/DllWazaOboe/0x82c.c
 */
int fn(void *this, void *unused, void *s) {
    (void)this;
    (void)unused;
    *((unsigned char *)s + 12) = 1;
    return 1;
}
