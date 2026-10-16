/* DllUSGymIce function @ CRO file offset 0x11f4, size 0x4 — MATCHED (masked, 1/1).
 * Trivial no-op void stub (single `bx lr`) — weak evidence, see note below.
 *
 * Any side-effect-free void function compiles to this same one word, so
 * the match alone doesn't prove the real body was empty, only that it's
 * at most this short (same caveat already documented for the
 * DllDialogCommon lone-`bx lr` stubs).
 *
 * Preceded by a 4-word literal pool (0x11e4-0x11f0) belonging to the
 * previous function (which ends at 0x11e0, `pop {r4,r5,pc}`), and
 * followed immediately by a new function whose first two instructions
 * are `add r1,r1,#4` then `push {r4,lr}` (register setup before the
 * prologue push — no spill needed before that point).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSGymIce 0x11f4 0x4 xy-decomp/armcc-matching/matched/DllUSGymIce/0x11f4.c
 */

void noop(void *this)
{
}
