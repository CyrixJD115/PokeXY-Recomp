/*
 * DllBattle @ 0xa8250, size 0x48 -- battle event trigger: when query(3)==c and query(57)>7, posts event 54 with helper(a)
 * Masked match 18/18. Family: event-trigger predicate around the 0x6b91c query-by-id getter; tail-calls the post-event veneer (0x4ce84 / 0x658d4).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xa8250 0x48 xy-decomp/armcc-matching/matched/DllBattle/0xa8250.c
 */
extern int g(int); extern int f4cbe4(int); extern void f4ce84(int,int);
void fn(int a,int b,int c){ if (g(3)==c && g(57)>7) f4ce84(54, f4cbe4(a)); }
