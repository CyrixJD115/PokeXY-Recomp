/* DllPuzzle @ 0x1e90 size 0x14 MATCHED (masked 5/5): post-indexed init: *o=v; callee(o+0x18); return callee_result-0x18.
 * retail has no r4 save: callee returns its arg, so write `return (char*)fn(p)-0x18` with fn returning void* (saving o in a local gives mov r4,r0 mismatch). Real size 0x14 not 0x18 (trailing word is padding)
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllPuzzle 0x1e90 0x14 xy-decomp/armcc-matching/matched/DllPuzzle/0x1e90.c
 */
extern void *fn388(void *p);
void *f(void *o, void *v)
{
    char *p = (char *)o;
    *(void **)p = v;
    p += 0x18;
    return (char *)fn388(p) - 0x18;
}
