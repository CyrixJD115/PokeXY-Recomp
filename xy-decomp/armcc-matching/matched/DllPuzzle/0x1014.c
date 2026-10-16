/*
 * DllPuzzle @ 0x1014, size 0x1c -- `if (p->f4->f2c->f60 == 1) p->f8 = 2;` (leaf, conditional execution moveq/strbeq).
 * Masked match 7/7. NOTE: the old attempted.md entry "DllPuzzle@0x100c" was a mis-offset: 0x100c..0x1013 are two zero
 * words (padding); the function starts at 0x1014 and retail IS conditionally executed, plain C matches.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllPuzzle 0x1014 0x1c xy-decomp/armcc-matching/matched/DllPuzzle/0x1014.c
 */
typedef struct C { char pad[0x60]; unsigned char f60; } C;
typedef struct D { char pad[0x2c]; C *f2c; } D;
typedef struct P { char pad[4]; D *f4; unsigned char f8; } P;
void fn(P *p)
{
    if (p->f4->f2c->f60 == 1)
        p->f8 = 2;
}
