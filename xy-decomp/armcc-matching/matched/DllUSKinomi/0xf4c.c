/* DllUSKinomi function @ CRO file offset 0xf4c, size 0x8 — MATCHED (masked, 2/2).
 * Trivial virtual-override stub: returns 0 (false), ignores `this`.
 *
 * First of three byte-identical `return 0` stubs in this cluster
 * (0xf4c/0xf54/0xf5c, all three matched in this same batch — see
 * `0xf54.c`/`0xf5c.c` in this directory). Sits right after two bare
 * `bx lr` no-op stubs (0xf44/0xf48) and a thin forwarder (0xf40) —
 * see `0xf40.c` for the full cluster layout.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSKinomi 0xf4c 0x8 xy-decomp/armcc-matching/matched/DllUSKinomi/0xf4c.c
 */

struct Obj;

int return0(struct Obj *this)
{
    return 0;
}
