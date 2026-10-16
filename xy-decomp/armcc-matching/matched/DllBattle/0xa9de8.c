/*
 * DllBattle @ 0xa9de8, size 0x14 -- forwards its four int args to the shared worker fn_4c284 with a fifth (stack) arg of 3; void tail wrapper.
 * Masked match 5/5. Constant-tweak sibling of the 0x4c5e0 / 0x4c284 wrapper family (only the fifth-arg constant differs).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xa9de8 0x14 xy-decomp/armcc-matching/matched/DllBattle/0xa9de8.c
 */
extern void fn_4c284(int, int, int, int, int);
void fn(int a, int b, int c, int d) { fn_4c284(a, b, c, d, 3); }
