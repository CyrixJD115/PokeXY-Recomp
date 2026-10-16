/*
 * DllBattle @ 0x1fc4, size 0x8 -- masked match 2/2.
 *
 * Trivial pointer-offset accessor: `return (char*)this + 2912;` -- adds a
 * fixed byte offset to `this` and returns it, no dereference (likely
 * returns the address of an embedded sub-object member).
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle \
 *     0x1fc4 0x8 xy-decomp/armcc-matching/matched/DllBattle/0x1fc4.c
 */

void *fn(char *o) {
    return o + 2912;
}
