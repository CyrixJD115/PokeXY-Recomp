/*
 * DllBattle @ 0xbc238, size 0x28 -- battle event-trigger predicate: if (g(a) == 12) post(50, 1352) (tail call); query id comes from the incoming arg.
 * Masked match 10/10. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xbc238 0x28 xy-decomp/armcc-matching/matched/DllBattle/0xbc238.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a) {
    if (g(a) == 12) fnA(50, 1352);
}
