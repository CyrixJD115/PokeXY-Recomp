/* DllSequence function @ CRO file offset 0x1140, size 0x18 — MATCHED (masked, 6/6).
 * Style: trick(pointer-walk) -- sequential pointer-cursor moves only to reproduce retail's immediate splits / pre-indexed stores
 * Sets a single byte flag deep inside a buffer owned via `this+20`:
 * `this->buf[0x181d] = 1;` — but the retail binary computes the address
 * as TWO separate immediate adds (`+0x1800` then `+29`) and stores with
 * a zero offset, rather than the more compact single-instruction form
 * (`add r0,r0,#0x1000; strb r1,[r0,#0x81d]`) that a flat `buf[0x181d]=1`
 * C expression naturally compiles to (the total offset 0x181d/6173
 * exceeds strb's 12-bit immediate range of 0-4095, so *some* split is
 * mandatory, but armcc's default choice folds as much as possible into
 * the strb's own offset field — only reproducing the retail split
 * required writing the pointer arithmetic as genuinely sequential
 * pointer increments through a local cursor variable (`p = buf; p +=
 * 0x1800; p += 29; *p = 1;`), matching pattern #5 from this toolchain's
 * residue notes (cursor-style stores produce different codegen than an
 * equivalent single-offset expression even though the end address is
 * identical).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllSequence 0x1140 0x18 xy-decomp/armcc-matching/matched/DllSequence/0x1140.c
 */

struct Outer
{
    unsigned char pad[20];
    unsigned char *buf;
};

void SetFlag(struct Outer *this)
{
    unsigned char *p = this->buf;
    p += 0x1800;
    p += 29;
    *p = 1;
}
