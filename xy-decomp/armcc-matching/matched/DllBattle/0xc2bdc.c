/*
 * DllBattle @ 0xc2bdc, size 0x60 -- battle event-trigger predicate: if (g(2) == c && g(3) != c && g(32) == 6 && g(33) < 0) *d = post(66, 1) (stores the veneer's result through d).
 * Masked match 24/24. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc2bdc 0x60 xy-decomp/armcc-matching/matched/DllBattle/0xc2bdc.c
 */
extern int g(int);
extern int f(int, int);
void fn(int a, int b, int c, int *d) {
    if (g(2) == c && g(3) != c && g(32) == 6 && g(33) < 0) *d = f(66, 1);
}
