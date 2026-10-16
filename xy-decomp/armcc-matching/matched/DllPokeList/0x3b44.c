/* DllPokeList @0x3b44, size 0x10 -- 4/4 masked match.
 * Style: natural
 *
 * Two-call void thunk: push {r4,lr}; bl A; pop {r4,lr}; b B -- calls one
 * function, then tail-calls the second. armcc folds the final call into
 * a bare b.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllPokeList 0x3b44 0x10 \
 *       xy-decomp/armcc-matching/matched/DllPokeList/0x3b44.c
 *   -> 4/4 match (b/bl targets masked) -> *** MASKED MATCH ***
 */
extern void a(void);
extern void b(void);

void fn_thunk(void)
{
    a();
    b();
}
