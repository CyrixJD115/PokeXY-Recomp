/*
 * DllBattle @ 0xbd17c, size 0x38 -- battle event-trigger predicate: if (g(3) == c) post(45, (uchar)(g(45) + 1)) (tail call): bumps query 45 by one.
 * Masked match 14/14. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xbd17c 0x38 xy-decomp/armcc-matching/matched/DllBattle/0xbd17c.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(3) == c) fnA(45, (unsigned char)(g(45) + 1));
}
