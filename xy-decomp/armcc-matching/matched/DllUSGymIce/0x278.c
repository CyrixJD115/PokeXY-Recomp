/* DllUSGymIce function @ CRO file offset 0x278, size 0x10 — MATCHED (masked, 4/4).
 * Two-level-indirection sign-extended byte getter: follows a pointer at
 * `this+20`, reads a word at `+16` of the pointee, returns it narrowed to
 * `signed char`.
 *
 * Non-obvious: a plain (non-volatile) `int field;` at that offset makes
 * armcc 5.04u5 at -O2 optimize the load straight down to `ldrb` (it can
 * see only the low byte survives the narrowing cast), which does NOT
 * match — the retail binary issues a full-word `ldr` and only narrows
 * afterward via `sxtb`. Marking the field `volatile` blocks that
 * optimization and reproduces the exact `ldr`+`sxtb` pair. (A local `int`
 * temp between the load and the cast was tried first and did not help —
 * only `volatile` did.)
 *
 * Sits right after the trivial-stub cluster (0x258-0x274) and right
 * before the module's first real function (a 6-register-push loop/
 * destructor-shaped helper) at 0x288.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSGymIce 0x278 0x10 xy-decomp/armcc-matching/matched/DllUSGymIce/0x278.c
 */

typedef struct Inner
{
    unsigned char pad[16];
    volatile int field;
} Inner;

typedef struct Outer
{
    unsigned char pad[20];
    Inner *inner;
} Outer;

signed char get_field(Outer *this)
{
    return (signed char)this->inner->field;
}
