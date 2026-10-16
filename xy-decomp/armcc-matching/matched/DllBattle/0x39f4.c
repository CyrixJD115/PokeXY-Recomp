/*
 * DllBattle @ 0x39f4, size 0xc -- this->+8 = 0 (setter). Masked match 3/3.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x39f4 0xc xy-decomp/armcc-matching/matched/DllBattle/0x39f4.c
 */
void fn(char *self) {
    *(int *)(self + 8) = 0;
}
