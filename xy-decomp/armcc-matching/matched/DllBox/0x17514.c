/* DllBox function @ CRO file offset 0x17514, size 0x30 -- MATCHED (masked, 12/12).
 * State-step: calls a 5-arg import with (ctx->h, 25, &o->b, ctx->f, 0) via
 * o->ctx (offset 80), then returns state id 38. Writing o->ctx twice (no
 * local) is what makes armcc hoist the stack-arg zero first, as retail does.
 * Style: natural -- plain member accesses, no tricks.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBox 0x17514 0x30 xy-decomp/armcc-matching/matched/DllBox/0x17514.c
 */
typedef struct { int h; unsigned char pad[1308]; int f; } Ctx;
typedef struct { unsigned char pad[80]; Ctx *ctx; unsigned char b[4]; } Obj;
extern void Call(int h, int a, void *p, int f, int z);
int Step(Obj *o)
{
    Call(o->ctx->h, 25, o->b, o->ctx->f, 0);
    return 38;
}
