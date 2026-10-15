/* DllSubScreenTutorial@0x12a0 (size 0x8) -- MATCHED (masked, 2/2).
 * Trivial virtual-override stub, `return 1`, ignores `this`.
 * One of a run of three identical "mov r0,#1; bx lr" stubs
 * (0x12a0/0x12a8/0x12b0, only this one claimed), followed by five bare
 * `bx lr` single-word stubs (0x12b8-0x12c8, weak-evidence class) right
 * before the next real function at 0x12cc.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllSubScreenTutorial 0x12a0 0x8 \
 *       xy-decomp/armcc-matching/matched/DllSubScreenTutorial/0x12a0.c
 */

int G(void *this)
{
    (void)this;
    return 1;
}
