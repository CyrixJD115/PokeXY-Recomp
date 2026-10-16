/*
 * DllBattle @ 0xc489c, size 0x60 -- battle event trigger: when event-id query(2)==c, query(3)!=c, query(32)==1 and query(33)<0, stores helper(66,1) through the out pointer
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc489c 0x60 xy-decomp/armcc-matching/matched/DllBattle/0xc489c.c
 */
extern int f6b91c(int);
extern int f658d4(int, int);
void fn(int a, int b, int c, int *d) {
    if (f6b91c(2) == c && f6b91c(3) != c && f6b91c(32) == 1 && f6b91c(33) < 0)
        *d = f658d4(66, 1);
}
