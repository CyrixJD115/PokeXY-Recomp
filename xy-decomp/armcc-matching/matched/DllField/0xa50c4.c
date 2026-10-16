/*
 * DllField @ 0xa50c4 (size 0x14) -- masked match 5/5.
 *
 * Passes the pointer field at this+344 to an import veneer and returns 0.
 * Call target is an import veneer (bl immediate is masked by the verifier).
 *
 * Style: natural -- plain C of the obvious shape; no tricks needed.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xa50c4 0x14 \
 *       xy-decomp/armcc-matching/matched/DllField/0xa50c4.c
 */
extern void H(void *);
int Fn(void *this) {
    H(*(void **)((char *)this + 344));
    return 0;
}
