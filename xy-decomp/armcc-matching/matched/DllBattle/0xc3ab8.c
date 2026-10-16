/*
 * DllBattle @ 0xc3ab8, size 0x48 -- battle event-trigger predicate: if (g(3) == c && h1((ushort)g(18)) == 1) post(54, 3277) (tail call); h1 = 0x7dc.
 * Masked match 18/18. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc3ab8 0x48 xy-decomp/armcc-matching/matched/DllBattle/0xc3ab8.c
 */
extern int g(int);
extern void fnA(int, int);
extern int h1(int);
void fn(int a, int b, int c) {
    if (g(3) == c && h1((unsigned short)g(18)) == 1) fnA(54, 3277);
}
