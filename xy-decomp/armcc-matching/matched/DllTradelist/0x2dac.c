/* DllTradelist @ 0x2dac, size 0xc - prologue-less tail-call forwarder: sets
 * up two literal args (r1=22, r2=1) and tail-branches to a local function,
 * passing the original first argument (r0) through unchanged. Match 3/3.
 *
 * Right after a small prologue'd helper ending at 0x2da8 (strb/mov/bx lr),
 * right before a larger push{r3,r4,r5,lr} dispatcher at 0x2dc4.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTradelist \
 *       0x2dac 0xc xy-decomp/armcc-matching/matched/DllTradelist/0x2dac.c
 */
extern int local_fn(void *a, int b, int c);

int fn(void *a) {
    return local_fn(a, 22, 1);
}
