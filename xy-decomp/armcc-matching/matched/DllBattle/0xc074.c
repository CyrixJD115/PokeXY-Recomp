/*
 * DllBattle @ 0xc074, size 0x30 -- virtual-override: ctx = *this; sets ctx bytes @0x7d8/@0x7d9 = 1 and ctx->@0x7e0 = ctx->@0x10->@0x40, calls import(10), returns 1. Masked match 12/12.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc074 0x30 xy-decomp/armcc-matching/matched/DllBattle/0xc074.c
 */
extern void fn_a(int v);
int fn(char *self) {
    char *b = *(char **)self;
    int t = (*(int **)(b + 0x10))[0x40 / 4];
    b[0x7d8] = 1;
    b[0x7d9] = 1;
    *(int *)(b + 0x7e0) = t;
    fn_a(10);
    return 1;
}
