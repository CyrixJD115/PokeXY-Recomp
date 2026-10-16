/*
 * DllBattle @ 0xc8f80, size 0x14 -- return (uint16)g(this->+0xc).
 * Masked match.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc8f80 0x14 xy-decomp/armcc-matching/matched/DllBattle/0xc8f80.c
 */
typedef struct O { int p[3]; void *x; } O;
extern unsigned g(void *);
unsigned short fn(O *o) { return g(o->x); }
