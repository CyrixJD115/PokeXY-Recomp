/*
 * DllBattle @ 0xa7af4, size 0x40 -- battle event trigger: when query(3)==c, copies query(31) into a local, calls modifier(&v,1) and posts event 31 with v
 * Masked match 16/16. Family: event-trigger predicate around the 0x6b91c query-by-id getter; tail-calls the post-event veneer (0x4ce84 / 0x658d4).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xa7af4 0x40 xy-decomp/armcc-matching/matched/DllBattle/0xa7af4.c
 */
extern int g(int); extern void f76ca8(int*,int); extern void f658d4(int,int);
void fn(int a,int b,int c){ if (g(3)==c){ int v=g(31); f76ca8(&v,1); f658d4(31,v);} }
