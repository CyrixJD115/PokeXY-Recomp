/* DllUSKinomi function @ CRO file offset 0xf40, size 0x4 — MATCHED (masked, 1/1).
 * Thin tail-call forwarder: entire body is a single `b <import veneer>`
 * instruction (the real target is 0x220 in the unlinked CRO, one of the
 * module's `ldr pc,[pc,#-4]` import-veneer slots; masked here as a
 * link-time fact).
 *
 * Sits right in the middle of a small stub cluster right after the
 * module's first real (large) function, which ends with a `bx lr` at
 * 0xf30: this forwarder at 0xf40, two bare `bx lr` no-op stubs at
 * 0xf44/0xf48, three 2-word `return 0` stubs at 0xf4c/0xf54/0xf5c (all
 * four of those also matched — see this directory), one more bare
 * `bx lr` at 0xf64, then the next real function (a big dispatcher)
 * starts at 0xf68. Same "thin forwarder" idiom as
 * `DllBattleVideoSave@0x454`/`DllBattleMatch@0xd90`/
 * `DllBattleVideoPlayer@0xb14`/`DllIntro@0x868`: a void->void tail call
 * with no other statements needs no stack frame, so armcc 5.04u5 at
 * -O2 -Otime folds it straight into a bare `b` instead of `bl`+
 * `pop {..,pc}`.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllUSKinomi 0xf40 0x4 xy-decomp/armcc-matching/matched/DllUSKinomi/0xf40.c
 */

extern void import_fn(void);

void forwarder(void)
{
    import_fn();
}
