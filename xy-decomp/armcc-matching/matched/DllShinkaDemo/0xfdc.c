/* DllShinkaDemo @ 0xfdc, size 0x8 -- masked match 2/2.
 *
 * Trivial signed-byte field getter: returns the signed byte at
 * this+0x82, no other side effects.
 *
 *   fdc: ldrsb r0, [r0, #130]
 *   fe0: bx lr
 *
 * Same shape as its immediate neighbor at 0xfe4 (identical getter for
 * the adjacent byte at +0x83, untried) -- part of the recurring
 * trivial-accessor cluster that sits right after a dispatcher function
 * (ending 0xfd4) and its one-word literal pool (0xfd8).
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllShinkaDemo 0xfdc 0x8 \
 *       xy-decomp/armcc-matching/matched/DllShinkaDemo/0xfdc.c
 */
signed char fn(void *obj) {
    return *(signed char *)((char *)obj + 0x82);
}
