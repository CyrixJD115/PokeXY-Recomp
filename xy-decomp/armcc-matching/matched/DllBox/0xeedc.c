/* DllBox@0xeedc, size 0x48, 18/18 masked match.
 * Check-then-act: if the import on (ctx->h->f4, 2, ctx->s24) returns 0 it
 * returns 1; otherwise it calls an import with (ctx->h, ctx->b, s24, s26)
 * and returns 0.
 * Style: natural
 * (Recovered from armcc-decomp-35's scratch file; the agent reported the
 * match but did not save it under matched/.)
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBox 0xeedc 0x48 xy-decomp/armcc-matching/matched/DllBox/0xeedc.c
 */
typedef struct { int pad; int f4; } H;
typedef struct { H *h; int pad; unsigned char b[1308]; unsigned short s24; unsigned short s26; } C;
typedef struct { unsigned char pad[80]; C *ctx; } Obj;
extern int Chk(int, int, int);
extern void Do(H *, void *, int, int);
int Step(Obj *o)
{
    C *c = o->ctx;
    if (Chk(c->h->f4, 2, c->s24) == 0)
        return 1;
    Do(c->h, c->b, c->s24, c->s26);
    return 0;
}
