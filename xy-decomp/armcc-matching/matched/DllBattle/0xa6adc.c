/*
 * DllBattle @ 0xa6adc, size 0x38 -- battle event trigger: when check(u8 query(2), c) holds, posts event 83 with arg 1
 * Masked match 14/14. Family: event-trigger predicate around the 0x6b91c query-by-id getter; tail-calls the post-event veneer (0x4ce84 / 0x658d4).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xa6adc 0x38 xy-decomp/armcc-matching/matched/DllBattle/0xa6adc.c
 */
extern int g(int); extern int f4c5a8(int,int); extern void f658d4(int,int);
void fn(int a,int b,int c){ if (f4c5a8((unsigned char)g(2), c)) f658d4(83,1); }
