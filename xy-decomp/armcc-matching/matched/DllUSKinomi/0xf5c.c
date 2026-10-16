/* DllUSKinomi function @ CRO file offset 0xf5c, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 0 (false), ignores `this`.
 *
 * Third of three byte-identical `return 0` stubs in this cluster
 * (0xf4c/0xf54/0xf5c) — see `0xf40.c` in this directory for the full
 * cluster layout. Right after this one, at 0xf64, is one more bare
 * `bx lr` no-op stub (untried), then the module's next real function
 * starts at 0xf68.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSKinomi 0xf5c 0x8 xy-decomp/armcc-matching/matched/DllUSKinomi/0xf5c.c
 */

struct Obj;

int return0(struct Obj *this)
{
    return 0;
}
