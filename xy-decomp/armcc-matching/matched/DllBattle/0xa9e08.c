/*
 * DllBattle @ 0xa9e08, size 0x14 -- forwarder: calls local function 0xa9e1c with the 4 incoming args plus a constant 5th stack arg 1.
 * Masked match.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xa9e08 0x14 xy-decomp/armcc-matching/matched/DllBattle/0xa9e08.c
 */
extern void callee(int, int, int, int, int);
void fn(int a, int b, int c, int d) { callee(a, b, c, d, 1); }
