/*
 * DllBattle @ 0x86734, size 0x24 -- script-command handler: reads one int argument from the script context and passes it with the battle object to a worker (fnB(b, v)).
 * Masked match 9/9.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x86734 0x24 xy-decomp/armcc-matching/matched/DllBattle/0x86734.c
 */
extern void rd(int, int*, int);
extern void fnB(int, int);
void fn(int a, int b) { int v; rd(a, &v, 1); fnB(b, v); }
