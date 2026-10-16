/*
 * DllBattle @ 0xc15f8, size 0x50 -- battle event-trigger predicate: if (g(3) == c && g(22) == 14 && *d != 0) post(50, 5325) (tail call, no return value); *d is a once-flag word.
 * Masked match 20/20. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc15f8 0x50 xy-decomp/armcc-matching/matched/DllBattle/0xc15f8.c
 */
extern int g(int);
extern void fnA(int, int);
void fn(int a, int b, int c, int *d) {
    if (g(3) == c && g(22) == 14 && *d != 0) fnA(50, 5325);
}
