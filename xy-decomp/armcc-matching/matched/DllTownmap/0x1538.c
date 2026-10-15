/* DllTownmap function @ CRO file offset 0x1538, size 0x1c - MATCHED (masked, 7/7).
 * if (this->[0x24] && that->[0x14]) tail-call fn(that->[0x14]); void.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTownmap 0x1538 0x1c xy-decomp/armcc-matching/matched/DllTownmap/0x1538.c
 */

extern void fn340(void *p);
void call_if(char *this)
{
    int *o = *(int **)(this + 0x24);
    if (o && o[5]) fn340((void *)o[5]);
}
