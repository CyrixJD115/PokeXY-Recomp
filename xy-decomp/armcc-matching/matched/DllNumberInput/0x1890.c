/* DllNumberInput function @ CRO file offset 0x1890, size 0x4c — MATCHED (masked, 19/19). Notification dispatcher: calls a handler's veneer twice (flag 0 then tail-call flag 1), two early-outs sharing one epilogue.
 * Style: natural
 *
 * Reverse-engineering note: the two early-return paths (null handle, and
 * handle already notified) share a SINGLE `pop {r4, pc}` epilogue with no
 * `mov r0, #N` before it — armcc reuses whatever is already live in r0 at
 * the branch site instead of re-materializing the return constant. The
 * null-handle path falls through the `cmp r0, #0` with r0 still 0, so
 * that path returns 0; the already-notified path falls through `cmp r0,
 * #1` with r0 still holding the compared flag byte (1), so that path
 * returns 1. Writing both early returns as literal `return 1;` (the
 * semantically "obvious" guess) cost an extra word versus the retail
 * binary — matching required returning the value already sitting in the
 * register, not the value that "means" true/false to a reader.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllNumberInput 0x1890 0x4c xy-decomp/armcc-matching/matched/DllNumberInput/0x1890.c
 */

typedef struct Inner { void *f0; void *f4; unsigned char flag8; } Inner;
typedef struct N { unsigned char pad[0x5c]; Inner **h5c; } N;
typedef struct T { unsigned char pad[0x24]; N *n24; } T;

extern int fn390(void *a, void *b, int c);

int F(T *t)
{
    Inner **hp;
    Inner *o;
    hp = t->n24->h5c;
    if (!hp)
        return 0;
    o = *hp;
    if (o->flag8 == 1)
        return 1;
    fn390(o->f4, o->f0, 0);
    return fn390(o->f4, o->f0, 1);
}
