/* DllUSGymEsper@0x328 (size 0x14) -- boolean getter through one pointer
 * indirection: `return this->p20->b == 1;`.
 *
 * Same idiom as DllStrInput@0x6674 (boolean getter through one pointer
 * indirection), but comparing against the literal 1 instead of testing
 * non-zero -- armcc reuses the already-loaded byte value as the true-case
 * result (it already equals 1) and only needs a conditional clear on the
 * false path, hence a lone `movne` with no matching `moveq`.
 *
 * Match: 5/5 masked words.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllUSGymEsper 0x328 0x14 \
 *       xy-decomp/armcc-matching/matched/DllUSGymEsper/0x328.c
 */

typedef struct Inner { unsigned char b; } Inner;
typedef struct T { unsigned char pad[20]; Inner *p20; } T;

int F(T *t)
{
    return t->p20->b == 1;
}
