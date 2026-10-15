/*
 * DllBattle @ 0x1fbc, size 0x8 -- masked match 2/2.
 *
 * Trivial virtual-override stub, `return 0`, ignores `this`. Same
 * recurring shape mined in nearly every other Dll module's stub cluster.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle \
 *     0x1fbc 0x8 xy-decomp/armcc-matching/matched/DllBattle/0x1fbc.c
 */

int fn(void *o) {
    return 0;
}
