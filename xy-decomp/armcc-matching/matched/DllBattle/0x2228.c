/*
 * DllBattle @ 0x2228, size 0x24 -- virtual call through this->+0x24 (vtbl slot 3); returns 1 if result==0 else 0.
 * Written as if/return (the `!x` form emits movne/moveq in swapped order).
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x2228 0x24 xy-decomp/armcc-matching/matched/DllBattle/0x2228.c
 */
typedef struct O { void **vt; } O;
int fn(char *self) {
    O *o = *(O **)(self + 0x24);
    int r = ((int (*)(O *))o->vt[3])(o);
    if (r != 0) return 0;
    return 1;
}
