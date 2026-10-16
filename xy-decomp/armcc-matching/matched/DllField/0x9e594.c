/*
 * DllField @ 0x9e594 (size 0x10) -- masked match 4/4.
 *
 * Calls a no-arg import veneer (result discarded) and returns 0.
 * Call target is an import veneer (bl immediate is masked by the verifier).
 *
 * Style: natural -- plain C of the obvious shape; no tricks needed.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9e594 0x10 \
 *       xy-decomp/armcc-matching/matched/DllField/0x9e594.c
 */
extern void H(void);
int Fn(void *this) {
    H();
    return 0;
}
