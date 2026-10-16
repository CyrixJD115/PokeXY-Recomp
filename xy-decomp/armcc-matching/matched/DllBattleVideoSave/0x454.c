/* DllBattleVideoSave @0x454, size 0x4 -- 1/1 masked match.
 * Style: natural
 *
 * Prologue-less tail-call forwarder: the entire function body is a single
 * `b <import veneer>` instruction (no push/pop, no other work), sitting
 * right after the big dispatcher function at 0x3f0-0x448 and its 2-word
 * literal pool at 0x44c/0x450. Likely a virtual-override stub that just
 * forwards straight to the real implementation reached via the module's
 * import-veneer table (target 0x190, one of the `ldr pc,[pc,#-4]` veneers
 * at the head of .text) -- same "thin forwarder" idiom as
 * DllDialogCommon@0x6e8 (`bx r1`), but through a fixed branch instead of
 * an indirect one.
 *
 * Reproducing this in C: a void->void tail call with no other statements
 * needs no stack frame, so armcc 5.04u5 at -O2 -Otime turns the call into
 * a direct `b` to the callee instead of `bl` + `bx lr`. b's 24-bit
 * immediate is masked by the verifier (link-time fact), so the exact
 * veneer target is unverified -- but the single-instruction tail-call
 * *shape* is proven.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllBattleVideoSave 0x454 0x4 \
 *       xy-decomp/armcc-matching/matched/DllBattleVideoSave/0x454.c
 *   -> target 1 words, candidate 1 words, 1/1 match (b/bl targets masked)
 *   -> *** MASKED MATCH ***
 */
extern void import190(void);

void fn454(void)
{
    import190();
}
