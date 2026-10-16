/*
 * DllBattle @ 0x71a2c, size 0x2c -- conditional store of constant 0x12c6 into c->v (+0x6c, c = arg3),
 * then returns (f394b8(a, k, c) != 0).
 * POOL-PAD match (all 8 code words identical, same single pool constant): retail starts at 4 mod 8, so
 * its literal pool sits after one 0 alignment word and the pc-relative ldr displacement is 4 larger than
 * a standalone compile's. Not a plain MASKED MATCH -- see README "Pool-pad tier".
 * Style: natural -- (`k` is also the value passed on to the callee, so the local is ordinary; the literal-inlined form does not match)
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x71a2c 0x2c xy-decomp/armcc-matching/poolpad/DllBattle/0x71a2c.c --pool-pad
 */
typedef struct C { char pad[0x6c]; int v; } C;
extern int f394b8(void *a, int k, C *c);
int fn(void *a, int unused, C *c) {
    int k = 0x12c6;
    if (c->v != k) c->v = k;
    return f394b8(a, k, c) != 0;
}
