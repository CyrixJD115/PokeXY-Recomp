/*
 * DllBattle @ 0x86bcc, size 0x2c -- script-command handler: reads a 16-byte vector argument (stack buffer) from the script context and hands it to a worker with the battle object.
 * Masked match 11/11.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x86bcc 0x2c xy-decomp/armcc-matching/matched/DllBattle/0x86bcc.c
 */
extern void rd(int, int*, int);
extern void fnB(int, int*);
void fn(int a, int b) { int v[4]; rd(a, v, 1); fnB(b, v); }
