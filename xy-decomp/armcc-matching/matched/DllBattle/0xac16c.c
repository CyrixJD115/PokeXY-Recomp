/*
 * DllBattle @ 0xac16c, size 0x34 -- battle event trigger: when lookup(u8 query(2)) == c, posts event 54 with 0x2000
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xac16c 0x34 xy-decomp/armcc-matching/matched/DllBattle/0xac16c.c
 */
extern int f6b91c(int);
extern int f4bfc0(int);
extern void f4ce84(int, int);
void fn(int a, int b, int c) {
    if (f4bfc0((unsigned char)f6b91c(2)) == c) f4ce84(54, 0x2000);
}
