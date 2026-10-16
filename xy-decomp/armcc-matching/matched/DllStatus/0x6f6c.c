/*
 * DllStatus @ 0x6f6c (size 0x4) -- masked match 1/1.
 * Thin tail-call forwarder: entire function body is a single `b <import
 * veneer>` instruction (target 0x310, inside this module's own
 * import-veneer table). A void->void tail call with no other statements
 * needs no stack frame, so armcc 5.04u5 at -O2 -Otime folds the call
 * straight into a bare `b` instead of `bl`+`pop {..,pc}` -- same idiom
 * already documented for DllBattleVideoSave@0x454 / DllBattleMatch@0xd90 /
 * DllBattleVideoPlayer@0xb14 / DllIntro@0x868.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStatus 0x6f6c 0x4 xy-decomp/armcc-matching/matched/DllStatus/0x6f6c.c
 */
typedef struct Obj Obj;

extern void import_fn(Obj *this);

void fn(Obj *this)
{
    import_fn(this);
}
