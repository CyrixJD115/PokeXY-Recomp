/*
 * DllStatus @ 0x2e54 (size 0x20) -- masked match 8/8.
 * Forwards a member to a helper: f470(c, f468(this->f8)); arg c (3rd) moved to r4 across the call, tail-calls f470.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStatus 0x2e54 0x20 xy-decomp/armcc-matching/matched/DllStatus/0x2e54.c
 */
extern void *f468(void *);
extern void f470(void *, void *);
void fn(char *this, int b, void *c)
{
    f470(c, f468(*(void **)(this + 8)));
}
