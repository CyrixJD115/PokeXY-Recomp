/*
 * DllField @ 0x95454 (size 0x14) -- masked match 5/5.
 *
 * Calls two no-arg import veneers in sequence and returns 0.
 * Call target is an import veneer (bl immediate is masked by the verifier).
 *
 * Style: natural -- plain C of the obvious shape; no tricks needed.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x95454 0x14 \
 *       xy-decomp/armcc-matching/matched/DllField/0x95454.c
 */
extern void H1(void);
extern void H2(void);
int Fn(void *this) {
    H1();
    H2();
    return 0;
}
