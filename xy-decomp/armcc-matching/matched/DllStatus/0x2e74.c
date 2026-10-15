/*
 * DllStatus @ 0x2e74 (size 0x1c) -- masked match 7/7.
 * f470(f478(this->f8), c): result of first call becomes arg 0 of tail-called f470, 3rd arg c as arg 1.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStatus 0x2e74 0x1c xy-decomp/armcc-matching/matched/DllStatus/0x2e74.c
 */
extern void *f478(void *);
extern void f470(void *, void *);
void fn(char *this, int b, void *c)
{
    f470(f478(*(void **)(this + 8)), c);
}
