/*
 * DllBattle @ 0xac0d4, size 0x34 -- battle event trigger: when lookup((u8)query(2))==c, posts event 54 with 0x400
 * Masked match 13/13. Family: event-trigger predicate around the 0x6b91c query-by-id getter; tail-calls the post-event veneer (0x4ce84 / 0x658d4).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xac0d4 0x34 xy-decomp/armcc-matching/matched/DllBattle/0xac0d4.c
 */
extern int g(int); extern int f4bfc0(int); extern void f4ce84(int,int);
void fn(int a,int b,int c){ if (f4bfc0((unsigned char)g(2))==c) f4ce84(54,0x400); }
