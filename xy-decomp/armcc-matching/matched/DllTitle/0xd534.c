/* DllTitle function @ CRO file offset 0xd534, size 0x18 -- MASKED MATCH 6/6.
 * Style: trick(split-expression) -- expression split across statements only to stop armcc fusing the load (ldrsh)
 * Fourth matched function from this module. Field-copy helper, no calls:
 * copies a byte from src+0 (zero-extended) into this+4, and a signed
 * 16-bit halfword from src+2 into this+0 (sign-extended to a full int
 * store). No virtual calls or branches.
 *
 * Non-obvious: a direct `(int)*(short*)(src+2)` cast compiles to a fused
 * `ldrsh` (2 instructions total with the store), which does NOT match --
 * the retail binary loads the halfword unsigned (`ldrh`) and sign-extends
 * it separately (`sxth`), 3 instructions. Reproducing that exact
 * instruction split required splitting the read into two statements: load
 * into an `unsigned short` temp first, then cast-and-store as a second
 * statement. A single combined expression gets fused by armcc into
 * `ldrsh`+`str` and does not match.
 *
 * Verify:
 *   XY_ROMFS=.Data/Raw_Extracted/romfs python3 \
 *     xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTitle 0xd534 0x18 \
 *     xy-decomp/armcc-matching/matched/DllTitle/0xd534.c
 */

void copy_fields(void *this, void *src)
{
    unsigned short t;
    *(int *)((char *)this + 4) = *(unsigned char *)src;
    t = *(unsigned short *)((char *)src + 2);
    *(int *)this = (short)t;
}
