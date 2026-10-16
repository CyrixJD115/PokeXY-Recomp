/*
 * DllBattle @ 0x16bf0, size 0x34 -- virtual-override: calls import(this+0x88, this, 0) then import(this+0x428, this, 1) (two embedded sub-objects bound to this), returns 1. Masked match 13/13.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x16bf0 0x34 xy-decomp/armcc-matching/matched/DllBattle/0x16bf0.c
 */
extern void fn_a(void *p, void *q, int v);
int fn(char *self) {
    fn_a(self + 0x88, self, 0);
    fn_a(self + 0x428, self, 1);
    return 1;
}
