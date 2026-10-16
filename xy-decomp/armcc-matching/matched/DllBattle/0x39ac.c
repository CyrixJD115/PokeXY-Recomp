/*
 * DllBattle @ 0x39ac, size 0x14 -- tail call fa(2, 0, 5, 0). Masked match 5/5.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x39ac 0x14 xy-decomp/armcc-matching/matched/DllBattle/0x39ac.c
 */
extern void fa(int, int, int, int);
void fn(void) {
    fa(2, 0, 5, 0);
}
