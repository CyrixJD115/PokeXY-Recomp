/*
 * DllBattle @ 0xa6494, size 0x20 -- forwarder: fnX(a, b, c, d, 2, 0) (r0-r3 passed through, two constants on the stack).
 * Masked match 8/8. Constant-tweak sibling of the 0xa6120 family; callee is a veneer (bl masked).
 * Style: trick(redundant-local) -- the 5th stack arg goes through a named local only to make armcc emit retail's `mov r4,#X; mov ip,#Y` order
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xa6494 0x20 xy-decomp/armcc-matching/matched/DllBattle/0xa6494.c
 */
extern void fnX(int, int, int, int, int, int);
void fn(int a, int b, int c, int d) {
    int e = 2;
    fnX(a, b, c, d, e, 0);
}
