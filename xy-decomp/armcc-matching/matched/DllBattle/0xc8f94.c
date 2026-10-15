/*
 * DllBattle @ 0xc8f94, size 0x20 -- virtual bool wrapper: return vt[0x60](this) != 4 (1/0).
 * Masked match 8/8.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc8f94 0x20 xy-decomp/armcc-matching/matched/DllBattle/0xc8f94.c
 */
typedef struct O { int (**vt)(struct O *); } O;
int fn(O *o) {
    if (o->vt[24](o) != 4) return 1;
    return 0;
}
