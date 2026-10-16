/*
 * DllBattle @ 0xa63ac, size 0x38 -- battle event trigger: when query(3)==c, posts event 45 with (u8)(query(45)+1)
 * Masked match 14/14. Family: event-trigger predicate around the 0x6b91c query-by-id getter; tail-calls the post-event veneer (0x4ce84 / 0x658d4).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xa63ac 0x38 xy-decomp/armcc-matching/matched/DllBattle/0xa63ac.c
 */
extern int g(int); extern void f658d4(int,int);
void fn(int a,int b,int c){ if (g(3)==c) f658d4(45,(unsigned char)(g(45)+1)); }
