/* DllUSGymFighting function @ CRO file offset 0x3f4, size 0x18 — MATCHED (masked, 6/6).
 * Conditional accessor: if this->field_at_16 == 3, tail-calls an import
 * veneer (0x1b8) on this->field_at_4 and returns the veneer's result;
 * otherwise returns `this` unchanged.
 *
 * Same idiom already documented across the swarm (DllLangSelect@0x5b0,
 * DllDendouDemo@0x798, DllBoxSearch@0x5b8, DllDendouPC@0x650,
 * DllEggDemo@0x568, DllPCMenu@0x6b8) -- a veneer call gates between a
 * member load and a fallback value -- except here the fallback is `this`
 * itself (not a re-materialized literal), and the taken branch is a tail
 * call (`b`, not `bl`) so no return-value handling is needed after it.
 *
 * Size gotcha: naively reading the body as "ldr/cmp/bne/ldr/b" alone
 * suggests 0x14 bytes, but the real function includes its own trailing
 * `bx lr` (the `bne` fallback target) as part of its body, making the
 * true size 0x18 -- the next function's own `bx lr` stub starts
 * immediately after, at 0x40c.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSGymFighting 0x3f4 0x18 xy-decomp/armcc-matching/matched/DllUSGymFighting/0x3f4.c
 */

extern void *veneer(void *);

struct S {
    int pad0;
    void *f4;
    int pad8;
    int pad12;
    int f16;
};

void *fn(struct S *this)
{
    if (this->f16 == 3) {
        return veneer(this->f4);
    }
    return this;
}
