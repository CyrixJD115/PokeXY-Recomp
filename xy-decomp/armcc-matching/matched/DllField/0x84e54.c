/*
 * DllField @ 0x84e54 (size 0x18) -- masked match 6/6.
 *
 * Bool getter: returns (Import(this->f12) != 0), i.e. loads the pointer at offset 12, passes it to an import veneer and normalises the result to 0/1 (cmp/movne).
 * Call target is an import veneer (bl immediate is masked by the verifier).
 *
 * Style: natural -- plain C of the obvious shape; no tricks needed.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x84e54 0x18 \
 *       xy-decomp/armcc-matching/matched/DllField/0x84e54.c
 */

extern int X(void *);
int F(void **t) { return X(t[3]) != 0; }
