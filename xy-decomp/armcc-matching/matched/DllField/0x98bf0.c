/*
 * DllField @ 0x98bf0 (size 0x10) -- masked match 4/4.
 *
 * Negated predicate: calls an import veneer with this unchanged and returns its result XOR 1 (logical not of a boolean getter).
 * Call target is an import veneer (bl immediate is masked by the verifier).
 *
 * Style: natural -- plain C of the obvious shape; no tricks needed.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x98bf0 0x10 \
 *       xy-decomp/armcc-matching/matched/DllField/0x98bf0.c
 */

extern int X(void *);
int F(void *t) { return X(t) ^ 1; }
