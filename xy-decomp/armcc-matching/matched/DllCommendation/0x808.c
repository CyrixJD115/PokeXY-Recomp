/* DllCommendation function @ CRO file offset 0x808, size 0x24 -- MATCHED (masked, 9/9). Calls an import with arg; on nonzero sets byte flag at +0x88 and returns 0, else returns 1.
 * Style: natural -- plain C
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllCommendation 0x808 0x24 xy-decomp/armcc-matching/matched/DllCommendation/0x808.c
 */

typedef struct X {
    unsigned char pad[0x88];
    unsigned char flag;      /* +0x88 */
} X;
extern int imp_808(void *a);

int f(X *o, void *a)
{
    if (imp_808(a) != 0) {
        o->flag = 1;
        return 0;
    }
    return 1;
}
