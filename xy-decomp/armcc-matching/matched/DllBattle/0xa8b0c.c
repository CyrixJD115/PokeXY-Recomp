/*
 * DllBattle @ 0xa8b0c, size 0x3c -- battle event trigger: when query(4)==c and query(27)==2, posts event 54 with 0x1800
 * Masked match 15/15. Family: event-trigger predicate around the 0x6b91c query-by-id getter; tail-calls the post-event veneer (0x4ce84 / 0x658d4).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xa8b0c 0x3c xy-decomp/armcc-matching/matched/DllBattle/0xa8b0c.c
 */
extern int g(int); extern void f4ce84(int,int);
void fn(int a,int b,int c){ if (g(4)==c && g(27)==2) f4ce84(54,0x1800); }
