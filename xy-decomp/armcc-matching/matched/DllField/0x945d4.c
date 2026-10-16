/*
 * DllField @ 0x945d4 (size 0x10) -- masked match 4/4.
 *
 * Forwards this to an import veneer (result discarded) and returns 0.
 * Call target is an import veneer (bl immediate is masked by the verifier).
 *
 * Style: natural -- plain C of the obvious shape; no tricks needed.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x945d4 0x10 \
 *       xy-decomp/armcc-matching/matched/DllField/0x945d4.c
 */
extern void H(void *);
int Fn(void *this) {
    H(this);
    return 0;
}
