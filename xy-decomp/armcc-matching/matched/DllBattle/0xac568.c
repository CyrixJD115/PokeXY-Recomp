/*
 * DllBattle @ 0xac568, size 0x34 -- battle event trigger: when lookup((u8)query(4))==c, posts event 66 with 1
 * Masked match 13/13. Family: event-trigger predicate around the 0x6b91c query-by-id getter; tail-calls the post-event veneer (0x4ce84 / 0x658d4).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xac568 0x34 xy-decomp/armcc-matching/matched/DllBattle/0xac568.c
 */
extern int g(int); extern int f4bfc0(int); extern void f658d4(int,int);
void fn(int a,int b,int c){ if (f4bfc0((unsigned char)g(4))==c) f658d4(66,1); }
