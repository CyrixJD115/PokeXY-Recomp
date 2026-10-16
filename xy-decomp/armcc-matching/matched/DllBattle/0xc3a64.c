/*
 * DllBattle @ 0xc3a64, size 0x54 -- battle event trigger: when query(3)==c and a lookup on (u16)query(18) returns 1, posts event 52 with helper(query(52), 0x1800)
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc3a64 0x54 xy-decomp/armcc-matching/matched/DllBattle/0xc3a64.c
 */
extern int f6b91c(int);
extern int f7dc(int);
extern int f9e708(int, int);
extern void f658d4(int, int);
void fn(int a, int b, int c) {
    if (f6b91c(3) == c && f7dc((unsigned short)f6b91c(18)) == 1)
        f658d4(52, f9e708(f6b91c(52), 0x1800));
}
