/*
 * DllPssAssistPower @ 0x618, size 0x8 — masked match 2/2.
 *
 * Trivial virtual-override stub, `return 1`, ignores `this`. Sits right
 * after the import-veneer table (0x180-0x617) in a cluster of near-
 * identical leaf stubs: four `mov r0,#1; bx lr` stubs at
 * 0x618/0x620/0x628/0x630/0x640 (only 0x618 claimed), interleaved with
 * bare `bx lr` weak-evidence stubs at 0x638/0x63c/0x648/0x64c, then the
 * module's first real function (a 7-case jump-table dispatcher) at 0x650.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllPssAssistPower 0x618 0x8 \
 *       xy-decomp/armcc-matching/matched/DllPssAssistPower/0x618.c
 */
int fn(void *this) {
    return 1;
}
