/*
 * DllBattle @ 0xc349c, size 0x58 -- battle event trigger: when query(2)==c and (u16)query(30)==4, posts event 51 with helper(query(51)/2, 1)
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc349c 0x58 xy-decomp/armcc-matching/matched/DllBattle/0xc349c.c
 */
extern int f6b91c(int);
extern int f9e6fc(int, int);
extern void f658d4(int, int);
void fn(int a, int b, int c) {
    if (f6b91c(2) == c && (unsigned short)f6b91c(30) == 4)
        f658d4(51, f9e6fc(f6b91c(51) / 2, 1));
}
