/*
 * DllBattle @ 0xea24, size 0x18 -- virtual-override wrapper: calls import(this->@0x40, 0) and returns 1 (true). Masked match 6/6.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xea24 0x18 xy-decomp/armcc-matching/matched/DllBattle/0xea24.c
 */
extern void fn_a(void *p, int v);
int fn(char *self) {
    fn_a(*(void **)(self + 0x40), 0);
    return 1;
}
