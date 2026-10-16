/*
 * DllBattle @ 0x3c124, size 0x18 -- copy 8 bytes from arg3->+172 (two ints) into this->+0x730; this->+0x72c (u8) = arg4.
 * Masked match 6/6.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x3c124 0x18 xy-decomp/armcc-matching/matched/DllBattle/0x3c124.c
 */
typedef struct P { int a, b; } P;
typedef struct O { char pad[172]; P p; } O;
typedef struct S { char pad[1836]; unsigned char f; char q[3]; P p; } S;
void fn(S *s, int u, O *o, int f) { s->p = o->p; s->f = f; }
