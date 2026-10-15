/*
 * DllGts @ 0x5ad4 (size 0x1c) -- masked match 7/7.
 * Calls f4f4(**(this+16) via a local, *(int*)(this+48)) and returns constant 50.
 * Style: trick(ordering-local) -- two locals (`q`, `v`) only to fix load-register order; the one-liner form got the load registers wrong
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllGts 0x5ad4 0x1c xy-decomp/armcc-matching/matched/DllGts/0x5ad4.c
 */
extern void f4f4(void *, int);
int fn(char *this)
{
    void **q = *(void ***)(this + 16);
    int v = *(int *)(this + 48);
    f4f4(*q, v);
    return 50;
}
