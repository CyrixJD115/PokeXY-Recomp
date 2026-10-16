/* DllField function @ CRO file offset 0x7fb78, size 0x5c -- MATCHED (masked, 23/23).
 * initializer: zeroes two u16, 11 floats at +4..+0x2c, int +0x30, float +0x34, sets int +0x38 = 15, zeroes +0x3c/+0x40; size includes the pool word.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x7fb78 0x5c xy-decomp/armcc-matching/matched/DllField/0x7fb78.c
 */

struct S {
    unsigned short a, b;
    float f[11];
    int i0;
    float g;
    int n;
    int j0, j1;
};
void init(struct S *s)
{
    s->a = 0;
    s->b = 0;
    s->f[0] = 0.0f; s->f[1] = 0.0f; s->f[2] = 0.0f; s->f[3] = 0.0f;
    s->f[4] = 0.0f; s->f[5] = 0.0f; s->f[6] = 0.0f; s->f[7] = 0.0f;
    s->f[8] = 0.0f; s->f[9] = 0.0f; s->f[10] = 0.0f;
    s->i0 = 0;
    s->g = 0.0f;
    s->n = 15;
    s->j0 = 0;
    s->j1 = 0;
}
