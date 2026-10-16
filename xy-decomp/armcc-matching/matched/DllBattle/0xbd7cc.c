/*
 * DllBattle @ 0xbd7cc, size 0x30 -- battle event-trigger predicate: if (g(3) == c) post(54, 5325) (tail call).
 * Masked match 12/12. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xbd7cc 0x30 xy-decomp/armcc-matching/matched/DllBattle/0xbd7cc.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(3) == c) fnA(54, 5325);
}
