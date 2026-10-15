/* DllStartMenu @ 0x1bfc, size 0x8 — masked match 2/2.
 *
 * One-line: trivial virtual-override stub, `return 0`.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllStartMenu \
 *       0x1bfc 0x8 xy-decomp/armcc-matching/matched/DllStartMenu/0x1bfc.c
 */
int func_1bfc(void) { return 0; }
