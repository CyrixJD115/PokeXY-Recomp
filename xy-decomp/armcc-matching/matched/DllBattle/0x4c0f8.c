/*
 * DllBattle @ 0x4c0f8, size 0x2c -- percent-chance roll: returns fnB(g.field54, 100) < a (fnB looks like a bounded random draw on a global RNG object at +84), i.e. true with probability a/100.
 * Masked match 11/11.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x4c0f8 0x2c xy-decomp/armcc-matching/matched/DllBattle/0x4c0f8.c
 */
extern unsigned fnB(int a, int b);
struct G { char pad[84]; int v; };
extern struct G g;
int fn(unsigned a) { return fnB(g.v, 100) < a; }
