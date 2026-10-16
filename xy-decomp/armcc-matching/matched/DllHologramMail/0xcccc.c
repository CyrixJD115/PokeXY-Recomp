/* DllHologramMail function @ CRO file offset 0xcccc, size 0x24 — MATCHED (masked, 9/9).
 * Style: natural
 * Field-copy-and-call helper: loads `this+20` (a pointer), dereferences it at
 * +136 to get a second object, calls a local helper (file offset 0x776c, an
 * un-inlined local function reached via `bl`, target masked) with that
 * object and the literal 13, then stores the literal 5 into `this+28`
 * (byte field) before returning void.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllHologramMail 0xcccc 0x24 xy-decomp/armcc-matching/matched/DllHologramMail/0xcccc.c
 */

typedef struct {
    unsigned char pad0[20];
    void *f20;
    unsigned char pad1[4];
    unsigned char f28;
} T;

extern void *local_776c(void *p, long v);

void fn_cccc(T *t)
{
    void *p = *(void **)((char *)t->f20 + 136);
    local_776c(p, 13);
    t->f28 = 5;
}
