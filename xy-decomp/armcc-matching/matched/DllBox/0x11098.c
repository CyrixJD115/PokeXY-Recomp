/* DllBox function @ CRO file offset 0x11098, size 0x54 -- MATCHED (masked, 21/21).
 * Pair-wise table search: scans t[i], t[i+1] for i = start, +2 while < 12;
 * returns slot id (i+105 / i+106); on miss calls an error import and returns 105.
 * A do-while (no entry bounds check) matches; a for-loop adds a precheck.
 * Style: natural -- plain do-while search loop, no tricks.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBox 0x11098 0x54 xy-decomp/armcc-matching/matched/DllBox/0x11098.c
 */
extern void Fail(int, int, int);
int Find(unsigned i, int v, int *t)
{
    do {
        if (t[i] == v)
            return i + 105;
        if (t[i + 1] == v)
            return i + 106;
        i += 2;
    } while (i < 12);
    Fail(0, 0, 0);
    return 105;
}
