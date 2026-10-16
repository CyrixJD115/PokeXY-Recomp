/*
 * DllField @ 0x5ee58 (size 0x14) -- masked match 5/5.
 *
 * Virtual-override stub: ignores this, calls an import veneer with arg2->f4 (the second argument's field at offset 4) and returns 0.
 * Call target is an import veneer (bl immediate is masked by the verifier).
 *
 * Style: natural -- plain C of the obvious shape; no tricks needed.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5ee58 0x14 \
 *       xy-decomp/armcc-matching/matched/DllField/0x5ee58.c
 */

extern void X(void *);
int F(void *a, void **b) { X(b[1]); return 0; }
