/* DllUSBigFourDragon function @ CRO file offset 0x2cc, size 0x8 — MATCHED (masked, 2/2).
 *
 * Trivial virtual-override stub: unconditional `return 0`. Sits in the
 * recurring tail-of-import-veneer-table leaf-stub cluster seen in every
 * Dll module mined so far: a lone `bx lr` at 0x2c8, this stub at 0x2cc,
 * two more identical `return 0` stubs at 0x2d4/0x2dc (unclaimed), two more
 * bare `bx lr` at 0x2e4/0x2e8, then a prologue-less tail-virtual-call leaf
 * at 0x2ec, another bare `bx lr` at 0x304, then the module's first real
 * function (push {r4,r5,r6,lr}) at 0x308.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSBigFourDragon 0x2cc 0x8 xy-decomp/armcc-matching/matched/DllUSBigFourDragon/0x2cc.c
 */

int F(void *this)
{
    return 0;
}
