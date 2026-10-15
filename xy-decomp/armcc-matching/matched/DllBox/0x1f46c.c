/* DllBox function @ CRO file offset 0x1f46c, size 0x10 — MATCHED (masked, 4/4).
 * Call-through-then-tail-call wrapper: calls one import veneer (bl), then
 * tail-branches into a second import veneer (b) instead of returning.
 *
 * Flagged by find_candidates.py from Ghidra's decompiled C
 * (`FUN_001009e0(); FUN_00100258(); return;`) as a two-fixed-no-arg-calls
 * void function. The real shape is `push {r4,lr}; bl <veneer@0x9e0>;
 * pop {r4,lr}; b <veneer@0x258>` — both targets are import-veneer-table
 * entries (table spans 0x180-0x9e7 in this module), and the second call
 * compiles as a tail branch (no return-address restore for it) since it's
 * the function's last statement. `r4` is pushed/popped alongside `lr`
 * purely for AAPCS stack-pointer-alignment padding around the first call,
 * same as already documented for other modules' leaf wrappers — it is not
 * carrying a real saved value.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBox 0x1f46c 0x10 xy-decomp/armcc-matching/matched/DllBox/0x1f46c.c
 */

extern void fn_a(void);
extern void fn_b(void);

void call_then_tail(void)
{
    fn_a();
    fn_b();
}
