/*
 * DllStatus @ 0x2fb8 (size 0x20) -- masked match 8/8.
 * Previous-index with wrap: n = fnCount(this->f8); return i ? i-1 : n-1.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStatus 0x2fb8 0x20 xy-decomp/armcc-matching/matched/DllStatus/0x2fb8.c
 */
extern unsigned fnN(void *p);
unsigned fn(char *this, unsigned i)
{
    unsigned n = fnN(*(void **)(this + 8));
    if (i) return i - 1;
    return n - 1;
}
