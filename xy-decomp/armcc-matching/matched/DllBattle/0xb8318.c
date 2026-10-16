/*
 * DllBattle @ 0xb8318, size 0x28 -- battle event-trigger predicate: if (g(2) == 31) post(a) (tail call; forwards the incoming first arg, r1 left untouched).
 * Masked match 10/10. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xb8318 0x28 xy-decomp/armcc-matching/matched/DllBattle/0xb8318.c
 */
extern int g(int);
extern void fnA(int);
void fn(int a) {
    if (g(2) == 31) fnA(a);
}
