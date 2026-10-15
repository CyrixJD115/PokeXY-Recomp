/* DllUSKinomi function @ CRO file offset 0xf54, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 0 (false), ignores `this`.
 *
 * Second of three byte-identical `return 0` stubs in this cluster
 * (0xf4c/0xf54/0xf5c) — see `0xf40.c` in this directory for the full
 * cluster layout.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSKinomi 0xf54 0x8 xy-decomp/armcc-matching/matched/DllUSKinomi/0xf54.c
 */

struct Obj;

int return0(struct Obj *this)
{
    return 0;
}
