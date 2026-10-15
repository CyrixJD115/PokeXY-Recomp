/*
 * Style: trick(redundant-local) -- named literal/temp local only to steer armcc's constant-register choice
 * DllWazaOmoidashi @ 0x924 (size 0x14) — 5/5 masked match.
 *
 * Field setter: stores the 2nd argument (a halfword) into a sub-object
 * pointed to by this+0x3c at offset +14, then sets a byte flag at
 * this+0x49 to 1.
 *
 * Note: register allocation only reproduces the retail scheduling (both
 * values loaded before either store: ldr r3,[r0,#60]; mov r2,#1; then
 * strh; strb) when the constant `1` is bound to its own local variable
 * declared right after the sub-pointer load, before either store
 * statement. Writing the literal `1` inline at the strb call site (or
 * reordering the two store statements) makes armcc 5.04u5 interleave a
 * load with its own store instead of hoisting both loads first — same
 * residue shape as this project's other register-scheduling traps, just
 * for constant materialization order rather than vptr/member order.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllWazaOmoidashi 0x924 0x14 \
 *     xy-decomp/armcc-matching/matched/DllWazaOmoidashi/0x924.c
 */
void fn(void *this, short v)
{
    char *sub = *(char **)((char *)this + 0x3c);
    unsigned char one = 1;
    *(short *)(sub + 14) = v;
    *(unsigned char *)((char *)this + 0x49) = one;
}
