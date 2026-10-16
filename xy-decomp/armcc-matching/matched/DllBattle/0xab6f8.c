/*
 * DllBattle @ 0xab6f8, size 0x3c -- battle event trigger: when query(2)==c and query(51)>0, posts event 66 with 1
 * Masked match 15/15. Family: event-trigger predicate around the 0x6b91c query-by-id getter; tail-calls the post-event veneer (0x4ce84 / 0x658d4).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xab6f8 0x3c xy-decomp/armcc-matching/matched/DllBattle/0xab6f8.c
 */
extern int g(int); extern void f658d4(int,int);
void fn(int a,int b,int c){ if (g(2)==c && g(51)>0) f658d4(66,1); }
