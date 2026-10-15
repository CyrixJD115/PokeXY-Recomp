/* DllTownmap function @ CRO file offset 0xd84, size 0x10 - MATCHED (masked, 4/4).
 * Style: natural
 * tail-call thunk: fn(this->[0x5c], this->[0x58], 0).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTownmap 0xd84 0x10 xy-decomp/armcc-matching/matched/DllTownmap/0xd84.c
 */

extern void fn290(void *a, int b, int c);
void f(char *this)
{
    fn290(*(void **)(this + 0x5c), *(int *)(this + 0x58), 0);
}
