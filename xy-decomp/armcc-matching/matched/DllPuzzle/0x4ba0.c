/* DllPuzzle @ 0x4ba0 size 0x14 MATCHED (masked 5/5): same as 0x1e90 with +4.
 * Style: trick(void-ptr-passthrough) -- callee declared `void*` and result offset (`(char*)fn(p) - N`) only to avoid a spare `mov r4,r0`
 * same return-callee-result-minus-offset trick; real size 0x14
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllPuzzle 0x4ba0 0x14 xy-decomp/armcc-matching/matched/DllPuzzle/0x4ba0.c
 */
extern void *fn490(void *p);
void *f(void *o, void *v)
{
    void **p = (void **)o;
    *p++ = v;
    return (char *)fn490(p) - 4;
}
