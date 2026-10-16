/*
 * DllField @ 0xe4878 (size 0x10) -- masked match 4/4.
 *
 * Call+return helper: forwards this to an import veneer (void result discarded) and returns the constant 12.
 * Call target is an import veneer (bl immediate is masked by the verifier).
 *
 * Style: natural -- plain C of the obvious shape; no tricks needed.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xe4878 0x10 \
 *       xy-decomp/armcc-matching/matched/DllField/0xe4878.c
 */

extern void X(void *);
int F(void *t) { X(t); return 12; }
