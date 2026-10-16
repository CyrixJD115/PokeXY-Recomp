/* DllSubScreenTutorial@0x538 (size 0x10) -- MATCHED (masked, 4/4).
 * Prologue-less leaf: tail virtual call through a direct member pointer,
 * same idiom as DllDialogCommon@0x6e8 but with no null check and the
 * owned object at a direct offset (this+0x38) instead of nested through
 * a sub-object. Loads o = this->p38, then calls o->vtbl[9](o) as a true
 * tail call (bx r1, r0 already holds o from the member load).
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllSubScreenTutorial 0x538 0x10 \
 *       xy-decomp/armcc-matching/matched/DllSubScreenTutorial/0x538.c
 */

typedef struct O O;
struct O { int (**vtbl)(O *self); };
typedef struct T { unsigned char pad[0x38]; O *p38; } T;

int G(T *t)
{
    O *o = t->p38;
    return o->vtbl[9](o);
}
