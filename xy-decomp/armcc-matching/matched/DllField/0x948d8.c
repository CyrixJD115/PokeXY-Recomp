/*
 * DllField @ 0x948d8 (size 0x18) -- masked match 6/6.
 *
 * Field-chain getter: loads this->f68 (0x44), calls a no-arg import veneer for its side effect (result unused), then returns the byte at ((f68)->f144)+330.
 * Call target is an import veneer (bl immediate is masked by the verifier).
 *
 * Style: natural -- plain C of the obvious shape; no tricks needed.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x948d8 0x18 \
 *       xy-decomp/armcc-matching/matched/DllField/0x948d8.c
 */

extern void *G(void);
int F(char *t) {
    char *o = *(char **)(t + 0x44);
    G();
    return *(unsigned char *)(*(char **)(o + 0x90) + 330);
}
