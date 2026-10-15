/* DllKawaigari function @ CRO file offset 0x3fc6c, size 0x30 — MATCHED (masked, 12/12).
 * No-arg helper: conditionally flags a nested global pointer chain, then
 * unconditionally sets a second global byte flag; always returns 1.
 *
 * Verify: python3 tools/fingerprint_fn.py DllKawaigari 0x3fc6c 0x30 \
 *     ../matched/DllKawaigari/0x3fc6c.c
 */

/* Prologue-less leaf, no incoming-argument use (r0/r1 both set from
 * literals/immediates inside the body) — a genuinely zero-argument
 * function, not a stub. Two literal-pool globals (both reloc placeholders,
 * 0 pre-link) are read at this function's own tail: g_outer's address at
 * +0x28 (pc+32), g_flag's address at +0x14 (pc+12) — their word-size span
 * (0x3fc94/0x3fc98, both zero in the plaintext CRO) is included in the
 * quoted size (0x30 = 0x28 code + 2 literal words), same convention as
 * the destructor-idiom matches elsewhere in this tree. */
typedef struct Inner {
    unsigned char pad[0x24];
    int flag;             /* +0x24 */
} Inner;

typedef struct Mid {
    unsigned char pad[0x28];
    Inner *inner;         /* +0x28 */
} Mid;

typedef struct Outer {
    unsigned char pad[0x48];
    Mid *mid;              /* +0x48 */
} Outer;

extern Outer g_outer;
extern unsigned char g_flag;

int Foo(void)
{
    Inner *inner = g_outer.mid->inner;
    if (inner) {
        inner->flag = 1;
    }
    g_flag = 1;
    return 1;
}
