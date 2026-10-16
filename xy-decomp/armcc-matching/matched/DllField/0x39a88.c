/*
 * DllField @ 0x39a88 (size 0x10) -- masked match 4/4.
 *
 * Identical idiom and byte shape to DllField@0x34238/0x36d04 (same
 * module, same literal instruction words): FunA(); FunB(); with the
 * second call tail-called via `b`.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField \
 *     0x39a88 0x10 xy-decomp/armcc-matching/matched/DllField/0x39a88.c
 */

extern void FunA(void);
extern void FunB(void);

void Func(void) {
    FunA();
    FunB();
}
