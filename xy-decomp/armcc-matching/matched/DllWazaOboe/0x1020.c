/*
 * DllWazaOboe @ 0x1020, size 0x4, masked match 1/1.
 * Thin tail-call forwarder — entire body is a single `b <import veneer>`
 * instruction, same idiom as DllBattleVideoSave@0x454/DllBattleMatch@0xd90/
 * DllBattleVideoPlayer@0xb14/DllIntro@0x868. A void->void tail call with no
 * other statements needs no stack frame, so armcc 5.04u5 at -O2 -Otime
 * folds the call straight into a bare `b` instead of `bl`+`pop {..,pc}`.
 * Sits right after the preceding function's own literal-pool word (0x101c)
 * and right before a bare `bx lr` leaf (0x1024, matched separately).
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllWazaOboe \
 *       0x1020 0x4 xy-decomp/armcc-matching/matched/DllWazaOboe/0x1020.c
 */
extern void import_fn(void);

void fn(void) {
    import_fn();
}
