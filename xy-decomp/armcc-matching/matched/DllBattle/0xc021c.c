/*
 * DllBattle @ 0xc021c, size 0x3c -- battle event-trigger predicate: if (g(3) == c && (unsigned)g(49) <= 60) post(50, 6144) (tail call).
 * Masked match 15/15. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc021c 0x3c xy-decomp/armcc-matching/matched/DllBattle/0xc021c.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(3) == c && (unsigned)g(49) <= 60) fnA(50, 6144);
}
