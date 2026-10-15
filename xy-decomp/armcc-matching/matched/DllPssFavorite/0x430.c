/* DllPssFavorite @0x430, size 0x8 — MASKED MATCH 2/2.
 *
 * Trivial virtual-override stub: `return 1`, no arguments used.
 * Part of the recurring tail-of-import-veneer-table leaf-stub cluster
 * seen in every mined Dll module: the import-veneer table runs
 * 0x180-0x42f (84 entries), then this stub and a twin at 0x438, then
 * three bare `bx lr` single-word stubs at 0x440/0x444/0x448, then the
 * module's first real function (a constructor-shaped dispatcher calling
 * the veneer at 0x180) at 0x44c.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllPssFavorite 0x430 0x8 \
 *       xy-decomp/armcc-matching/matched/DllPssFavorite/0x430.c
 */
int fn(void *this)
{
    return 1;
}
