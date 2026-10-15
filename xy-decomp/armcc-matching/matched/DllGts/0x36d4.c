/*
 * DllGts @ 0x36d4 (size 0x18) -- masked match 6/6.
 * Calls f314(this+32), returns int field this+20.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllGts 0x36d4 0x18 xy-decomp/armcc-matching/matched/DllGts/0x36d4.c
 */
extern void f314(void *);
int fn(char *this)
{
    f314(this + 32);
    return *(int *)(this + 20);
}
