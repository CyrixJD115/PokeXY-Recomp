/*
 * DllBattle @ 0xc900c, size 0xc -- signed byte getter: returns (signed char) this->+0x875.
 * Masked match 3/3.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc900c 0xc xy-decomp/armcc-matching/matched/DllBattle/0xc900c.c
 */
typedef struct O { char pad[2165]; signed char v; } O;
int fn(O *o) { return o->v; }
