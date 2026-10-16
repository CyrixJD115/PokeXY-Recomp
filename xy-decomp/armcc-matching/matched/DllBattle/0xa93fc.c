/*
 * DllBattle @ 0xa93fc, size 0x14 -- singleton-stub: stores constant 4 through its first arg, returns the address of a module global (pool word, relocated at load; zero in the unlinked CRO).
 * Masked match 5/5. WEAK evidence (a tiny stub of this shape compiles identically whatever the real body); ONE representative of the (N=4, pool=reloc'd address) shape.
 * Sibling offsets with the identical shape: 15 more (see pending note).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xa93fc 0x14 xy-decomp/armcc-matching/matched/DllBattle/0xa93fc.c
 */
extern int G;
int *fn(int *p) { *p = 4; return &G; }
