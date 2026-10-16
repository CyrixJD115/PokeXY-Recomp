/*
 * DllBattle @ 0xbc29c, size 0x24 -- battle event-trigger predicate: if (g(a) == 0) post(22, 12) (tail call); query id comes from the incoming arg.
 * Masked match 9/9. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xbc29c 0x24 xy-decomp/armcc-matching/matched/DllBattle/0xbc29c.c
 */
extern int g(int);
extern void fnB(int, int);
void fn(int a) {
    if (g(a) == 0) fnB(22, 12);
}
