/* DllPuzzle @ 0xf04 size 0x2c MATCHED (masked 11/11): conditional bool: mode==1 && fn(obj) -> 1 else 0.
 * Style: natural
 * if(a) {if(r) return 1;} return 0 shape
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllPuzzle 0xf04 0x2c xy-decomp/armcc-matching/matched/DllPuzzle/0xf04.c
 */
typedef struct A { unsigned char pad[0x20]; unsigned char mode; unsigned char p2[3]; void *obj; } A;
extern int fn208(void *o);
int f(A *a)
{
    if (a->mode == 1) {
        if (fn208(a->obj)) return 1;
    }
    return 0;
}
