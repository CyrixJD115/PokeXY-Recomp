/* DllField function @ CRO file offset 0x29080, size 0x10 -- MATCHED (masked, 4/4).
 * vptr setter: stores a class vtable address (pool word, relocated at link) into this[0]; size includes the pool word.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x29080 0x10 xy-decomp/armcc-matching/matched/DllField/0x29080.c
 */

extern char VT[];

void set_vptr(void **p)
{
    *p = VT;
}
