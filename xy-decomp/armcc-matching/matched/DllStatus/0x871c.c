/*
 * DllStatus @ 0x871c (size 0x4) -- masked match 1/1.
 * Thin tail-call forwarder: entire function body is a single `b <import
 * veneer>` instruction -- same idiom as DllStatus@0x6f6c (this module)
 * and DllBattleVideoSave@0x454/DllBattleMatch@0xd90/
 * DllBattleVideoPlayer@0xb14/DllIntro@0x868.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStatus 0x871c 0x4 xy-decomp/armcc-matching/matched/DllStatus/0x871c.c
 */
typedef struct Obj Obj;

extern void import_fn(Obj *this);

void fn(Obj *this)
{
    import_fn(this);
}
