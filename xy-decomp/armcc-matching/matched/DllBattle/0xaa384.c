/*
 * DllBattle @ 0xaa384, size 0x50 -- battle event trigger: when query(3)==c and lookup((u16)query(18))==1, posts event 50 with helper(a)
 * Masked match 20/20. Family: event-trigger predicate around the 0x6b91c query-by-id getter; tail-calls the post-event veneer (0x4ce84 / 0x658d4).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xaa384 0x50 xy-decomp/armcc-matching/matched/DllBattle/0xaa384.c
 */
extern int g(int); extern int f7dc(int); extern int f4cbe4(int); extern void f4ce84(int,int);
void fn(int a,int b,int c){ if (g(3)==c && f7dc((unsigned short)g(18))==1) f4ce84(50, f4cbe4(a)); }
