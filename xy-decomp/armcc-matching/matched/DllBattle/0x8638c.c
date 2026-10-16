/*
 * DllBattle @ 0x8638c, size 0x30 -- script-command handler: reads and discards two consecutive int arguments from the script context.
 * Masked match 12/12.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x8638c 0x30 xy-decomp/armcc-matching/matched/DllBattle/0x8638c.c
 */
extern void rd(int, int*, int);
void fn(int a) { int v, w; rd(a, &v, 1); rd(a, &w, 1); }
