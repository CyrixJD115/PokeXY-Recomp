/*
 * DllBattle @ 0xb86e4, size 0x30 -- battle event-trigger predicate: if (g(3) == *d) post(50, 6144) (tail call); compares the query with the word at d.
 * Masked match 12/12. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xb86e4 0x30 xy-decomp/armcc-matching/matched/DllBattle/0xb86e4.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c, int *d) {
    if (g(3) == *d) fnA(50, 6144);
}
