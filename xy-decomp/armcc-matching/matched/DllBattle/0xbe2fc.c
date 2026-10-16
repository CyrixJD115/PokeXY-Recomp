/*
 * DllBattle @ 0xbe2fc, size 0x44 -- battle event-trigger predicate: if (g(4) == c && h((unsigned char)g(57)) == 2) post(54, 3072) (tail call, no return value); h is an 0x9e58c lookup helper.
 * Masked match 17/17. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xbe2fc 0x44 xy-decomp/armcc-matching/matched/DllBattle/0xbe2fc.c
 */
extern int g(int);
extern int h(int);
extern void fnA(int, int);
void fn(int a, int b, int c) {
    if (g(4) == c && h((unsigned char)g(57)) == 2) fnA(54, 3072);
}
