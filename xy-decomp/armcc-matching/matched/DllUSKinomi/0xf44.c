/* DllUSKinomi function @ CRO file offset 0xf44, size 0x4 — MATCHED (masked, 1/1).
 * Trivial no-op void stub (single `bx lr`) — weak evidence: any
 * side-effect-free void function compiles to this exact one word, so
 * the match alone doesn't prove the real body was empty, only that
 * it's at most this short.
 *
 * Second of two identical lone `bx lr` stubs (0xf44/0xf48) between the
 * thin forwarder at 0xf40 and the `return 0` stub cluster starting at
 * 0xf4c — see `0xf40.c` in this directory for the full cluster layout.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSKinomi 0xf44 0x4 xy-decomp/armcc-matching/matched/DllUSKinomi/0xf44.c
 */

struct Obj;

void noop(struct Obj *this)
{
}
