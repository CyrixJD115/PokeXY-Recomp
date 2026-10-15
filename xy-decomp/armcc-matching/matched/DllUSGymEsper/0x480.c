/* DllUSGymEsper@0x480 (size 0x18) -- boolean setter through one pointer
 * indirection: normalizes an int argument to 0/1 and stores it through
 * the `this+20` pointer member's first byte.
 *
 * Sibling of the boolean getter at 0x328 (same `this+20` pointer member,
 * opposite direction). Source phrasing matters here: `v ? 1 : 0` /
 * `v != 0` directly assigned to the field made armcc reuse the argument
 * register in place (`cmp r1,#0; movne r1,#1; strb r1,[r0]` -- 5 words,
 * missing the target's explicit `mov r2,#0`). Writing it as a separate
 * `int flag = 0; if (v) flag = 1;` local, then storing that local,
 * reproduces the target's unconditional-zero-then-conditional-one
 * sequence through a distinct register (r2) exactly.
 *
 * Match: 6/6 masked words.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllUSGymEsper 0x480 0x18 \
 *       xy-decomp/armcc-matching/matched/DllUSGymEsper/0x480.c
 */

typedef struct Inner { unsigned char b; } Inner;
typedef struct T { unsigned char pad[20]; Inner *p20; } T;

void F(T *t, int v)
{
    int flag = 0;
    if (v != 0)
        flag = 1;
    t->p20->b = (unsigned char)flag;
}
