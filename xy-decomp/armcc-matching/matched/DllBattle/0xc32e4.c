/*
 * DllBattle @ 0xc32e4, size 0x58 -- battle event trigger: when query(3)==c and (u16)query(18) is one of 0x160/0x18f/0x196/0x18c, posts event 50 with 0x1800
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc32e4 0x58 xy-decomp/armcc-matching/matched/DllBattle/0xc32e4.c
 */
extern int f6b91c(int);
extern void f4ce84(int, int);
void fn(int a, int b, int c) {
    if (f6b91c(3) == c) {
        unsigned short v = f6b91c(18);
        if (v == 0x160 || v == 0x18f || v == 0x196 || v == 0x18c) f4ce84(50, 0x1800);
    }
}
