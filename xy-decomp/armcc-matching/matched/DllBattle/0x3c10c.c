/*
 * DllBattle @ 0x3c10c, size 0x18 -- copy 8 bytes from arg3->+164 (two ints) into this->+0x730; this->+0x72c (u8) = arg4.
 * Masked match 6/6.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x3c10c 0x18 xy-decomp/armcc-matching/matched/DllBattle/0x3c10c.c
 */
typedef struct P { int a, b; } P;
typedef struct O { char pad[164]; P p; } O;
typedef struct S { char pad[1836]; unsigned char f; char q[3]; P p; } S;
void fn(S *s, int u, O *o, int f) { s->p = o->p; s->f = f; }
