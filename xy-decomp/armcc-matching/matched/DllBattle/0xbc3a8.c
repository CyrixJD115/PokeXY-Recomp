/*
 * DllBattle @ 0xbc3a8, size 0x18 -- battle event-trigger predicate: post(62, g(a) + 1) (tail call): bumps the queried value by one.
 * Masked match 6/6. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xbc3a8 0x18 xy-decomp/armcc-matching/matched/DllBattle/0xbc3a8.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a) {
    fnA(62, g(a) + 1);
}
