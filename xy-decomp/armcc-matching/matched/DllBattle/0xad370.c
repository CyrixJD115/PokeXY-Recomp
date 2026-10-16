/*
 * DllBattle @ 0xad370, size 0x14 -- singleton-stub: stores constant 9 through its first arg, returns the address of a module global (pool word, relocated at load; zero in the unlinked CRO).
 * Masked match 5/5. WEAK evidence (a tiny stub of this shape compiles identically whatever the real body); ONE representative of the (N=9, pool=reloc'd address) shape.
 * Sibling offsets with the identical shape: 1 more (see pending note).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xad370 0x14 xy-decomp/armcc-matching/matched/DllBattle/0xad370.c
 */
extern int G;
int *fn(int *p) { *p = 9; return &G; }
