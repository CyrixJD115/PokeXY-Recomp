/*
 * Style: natural
 * DllNuts @ 0x3384, size 0xc, match 3/3 (masked).
 *
 * Trivial field setter: unconditionally zeroes a byte flag at `this+156`
 * (0x9c), ignoring any passed value, void return. Sits in a small run of
 * three bare `bx lr` single-word stubs (0x3378/0x337c/0x3380, not
 * claimed) right before it, between two real functions — same recurring
 * tail-of-vtable trivial-stub/setter cluster shape documented across
 * every other mined Dll module.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllNuts 0x3384 0xc xy-decomp/armcc-matching/matched/DllNuts/0x3384.c
 */
void Func(void *this)
{
    *(unsigned char *)((char *)this + 156) = 0;
}
