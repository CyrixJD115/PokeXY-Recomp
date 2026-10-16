/*
 * DllBattle @ 0x8070c, size 0x3c -- script-command handler: reads one float argument, substitutes 50.0f if it is negative, and stores it into the object's float field at +248.
 * Masked match 15/15. Float arg reader (veneer 0x1acc); the 0.0f compare constant and the 50.0f default both live in the literal pool (hence 15 words incl. 2 pool words).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x8070c 0x3c xy-decomp/armcc-matching/matched/DllBattle/0x8070c.c
 */
extern void rd(int, float*, int);
typedef struct O { char pad[248]; float f; } O;
void fn(int a, O *o) { float v; rd(a, &v, 1); if (v < 0.0f) v = 50.0f; o->f = v; }
