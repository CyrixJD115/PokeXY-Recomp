/*
 * DllStatus @ 0x4654 (size 0x30) -- masked match 12/12.
 * Loads u32 at ctx(arg3)+0x1dc, then calls two import veneers with
 * (this->p58, v, 0) and (this->p58, v, 1); the second is a tail call (`b`).
 * Arg 2 unused. Plain source order matches with no tricks.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStatus 0x4654 0x30 xy-decomp/armcc-matching/matched/DllStatus/0x4654.c
 */
typedef struct { unsigned char pad[88]; void *p58; } Obj;
typedef struct { unsigned char pad[476]; unsigned v; } Ctx;
extern void imp_a(void *p, unsigned v, int f);
extern void imp_b(void *p, unsigned v, int f);
void fn(Obj *this, int unused, Ctx *c)
{
    unsigned v = c->v;
    imp_a(this->p58, v, 0);
    imp_b(this->p58, v, 1);
}
