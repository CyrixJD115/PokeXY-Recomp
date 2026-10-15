/*
 * DllGts @ 0x486c (size 0x18) -- masked match 6/6.
 * Virtual-wrapper-style: f4ec(f4cc(this), c) with 3rd arg c kept in r4 across the call, tail-call.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllGts 0x486c 0x18 xy-decomp/armcc-matching/matched/DllGts/0x486c.c
 */
extern void *f4cc(void *);
extern void f4ec(void *, void *);
void fn(void *this, int b, void *c)
{
    f4ec(f4cc(this), c);
}
