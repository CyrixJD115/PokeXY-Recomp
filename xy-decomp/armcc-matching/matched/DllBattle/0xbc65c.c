/*
 * DllBattle @ 0xbc65c, size 0x24 -- battle event-trigger predicate: if (g(a) == 4) post(76, 1) (tail call).
 * Masked match 9/9. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xbc65c 0x24 xy-decomp/armcc-matching/matched/DllBattle/0xbc65c.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a) {
    if (g(a) == 4) fnA(76, 1);
}
