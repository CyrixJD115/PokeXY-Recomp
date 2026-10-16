/*
 * DllBattle @ 0xac02c, size 0x44 -- battle event trigger: when lookup((u8)query(3))==c, posts event 39 with (query(39)<<1)&0xffff
 * Masked match 17/17. Family: event-trigger predicate around the 0x6b91c query-by-id getter; tail-calls the post-event veneer (0x4ce84 / 0x658d4).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xac02c 0x44 xy-decomp/armcc-matching/matched/DllBattle/0xac02c.c
 */
extern int g(int); extern int f4bfc0(int); extern void f658d4(int,int);
void fn(int a,int b,int c){ if (f4bfc0((unsigned char)g(3))==c) f658d4(39,(g(39)<<1)&0xFFFF); }
