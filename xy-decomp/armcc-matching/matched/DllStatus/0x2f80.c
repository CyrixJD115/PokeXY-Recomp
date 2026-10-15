/*
 * DllStatus @ 0x2f80 (size 0x14) -- masked match 5/5.
 * Tail-call wrapper: calls fn(this->f16, b, a) with the two int args swapped.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStatus 0x2f80 0x14 xy-decomp/armcc-matching/matched/DllStatus/0x2f80.c
 */
extern void fnX(void *p, int a, int b);
void fn(char *this, int a, int b)
{
    fnX(*(void **)(this + 16), b, a);
}
