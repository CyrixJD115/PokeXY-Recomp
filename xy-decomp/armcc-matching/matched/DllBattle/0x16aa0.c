/*
 * DllBattle @ 0x16aa0, size 0x14 -- virtual-override wrapper: calls import(this, this->@0x10) and returns 1 (true). Masked match 5/5.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x16aa0 0x14 xy-decomp/armcc-matching/matched/DllBattle/0x16aa0.c
 */
extern void fn_a(void *p, void *q);
int fn(char *self) {
    fn_a(self, *(void **)(self + 0x10));
    return 1;
}
