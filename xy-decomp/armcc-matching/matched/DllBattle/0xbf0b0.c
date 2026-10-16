/*
 * DllBattle @ 0xbf0b0, size 0x4c -- battle event-trigger predicate: *d = 0; if (g(2) == c && g(22) == 0) *d = post(22, 17) (stores the veneer's result through d).
 * Masked match 19/19. Queries go through the 0x6b91c query-by-id getter; the post-event veneer is a tail call, no return value.
 * Style: trick(redundant-local) -- the post-event arg (17) is held in a callee-saved register (mov r6,#17) hoisted above the first query; the named local `k` reproduces that (audit 2026-10-16: inlining the literal fails to match)
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xbf0b0 0x4c xy-decomp/armcc-matching/matched/DllBattle/0xbf0b0.c
 */
extern int g(int);
extern int f(int, int);
void fn(int a, int b, int c, int *d) {
    int k = 17;
    *d = 0;
    if (g(2) == c && g(22) == 0) *d = f(22, k);
}
