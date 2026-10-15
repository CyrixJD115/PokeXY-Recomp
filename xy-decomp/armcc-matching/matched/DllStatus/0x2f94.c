/*
 * DllStatus @ 0x2f94 (size 0x24) -- masked match 9/9.
 * Next-index with wrap: n = fnCount(this->f8); return (i+1 < n) ? i+1 : 0.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStatus 0x2f94 0x24 xy-decomp/armcc-matching/matched/DllStatus/0x2f94.c
 */
extern unsigned fnN(void *p);
unsigned fn(char *this, unsigned i)
{
    unsigned n = fnN(*(void **)(this + 8));
    if (i + 1 < n) return i + 1;
    return 0;
}
