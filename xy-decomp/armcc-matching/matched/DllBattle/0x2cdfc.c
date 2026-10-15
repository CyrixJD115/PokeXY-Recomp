/*
 * DllBattle @ 0x2cdfc, size 0x8 -- tail call: fn(this->+0x12c) (void).
 * Masked match 2/2.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x2cdfc 0x8 xy-decomp/armcc-matching/matched/DllBattle/0x2cdfc.c
 */
typedef struct O { char pad[0x12c]; int x; } O;
extern void fa(int);
void fn(O *o) { fa(o->x); }
