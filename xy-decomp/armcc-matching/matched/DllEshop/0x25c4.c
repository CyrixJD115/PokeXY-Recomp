/* DllEshop function @ CRO file offset 0x25c4, size 0x10 — MATCHED (masked, 4/4).
 * Wrapper: calls an inner function with this, then tail-calls another
 * import with inner's result as its argument (r0 not preserved, so no
 * `mov r4,r0`: tail188(inner(o)) with inner returning void*).
 * Style: natural
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllEshop 0x25c4 0x10 xy-decomp/armcc-matching/matched/DllEshop/0x25c4.c
 */
extern void *inner(void *o);
extern void *tail188(void *o);
void *fn(void *o)
{
    return tail188(inner(o));
}
