/* DllField function @ CRO file offset 0x8604, size 0xc -- MATCHED (masked, 3/3).
 * Tail-calls an import veneer with (this->field@0xa4, 0); returns its result.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x8604 0xc xy-decomp/armcc-matching/matched/DllField/0x8604.c
 */

extern void *fn2e0(void *p, int x);
struct O { unsigned char pad[164]; void *m; };

void *Fn(struct O *o)
{
    return fn2e0(o->m, 0);
}
