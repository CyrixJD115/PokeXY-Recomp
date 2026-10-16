/*
 * DllBattle @ 0x227c, size 0x10 -- tail-calls virtual slot 5 of the object at this+0x24.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x227c 0x10 xy-decomp/armcc-matching/matched/DllBattle/0x227c.c
 */
typedef struct O { void **vt; } O;
int fn(char *self) {
    O *o = *(O **)(self + 0x24);
    return ((int (*)(O *))o->vt[5])(o);
}
