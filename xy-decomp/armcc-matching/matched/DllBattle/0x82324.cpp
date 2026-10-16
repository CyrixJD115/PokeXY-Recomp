/*
 * DllBattle @ 0x82324, size 0x14 -- returns !fn(o, p->a, p->b): calls a predicate with a two-word pair unpacked from p, then negates the bool result (C++ bool `!` compiles to eor #1).
 * Masked match, all words.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x82324 0x14 xy-decomp/armcc-matching/matched/DllBattle/0x82324.cpp
 */
typedef struct { int a, b; } P;
extern bool fn799d8(int, int, int);
bool fn(int o, P *p) { return !fn799d8(o, p->a, p->b); }
