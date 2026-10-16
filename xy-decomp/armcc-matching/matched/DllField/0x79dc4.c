/*
 * DllField @ 0x79dc4 (size 0x60) -- masked match 24/24, USING --mask-ranges 0x10:0x40 (16-word jump table).
 * Case-value-to-body mapping was RECOVERED from the CRO internal relocations (tools/jumptable_relocs.py
 * DllField 0x79dd4 0x79e14), not proven by the compiler: with idx = k-4, slots 0-3 and 8-15 -> "return 1",
 * slots 4-7 -> "return 0"; i.e. k in {4..7, 12..19} true, k in {8..11} and everything else false.
 * What it does: bool predicate over a small id range (leaf, no frame).
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x79dc4 0x60 xy-decomp/armcc-matching/matched/DllField/0x79dc4.c --mask-ranges 0x10:0x40
 */
int fn(int k)
{
    switch (k) {
    case 4: case 5: case 6: case 7:
    case 12: case 13: case 14: case 15: case 16: case 17: case 18: case 19:
        return 1;
    default:
        return 0;
    }
}
