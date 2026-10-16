/* DllDialogCommon function @ CRO file offset 0x8d0, size 0x2c.
 * Style: natural
 *
 * FIRST MATCHED FUNCTION of the project (2026-10-13): compiled with
 * armcc 5.04u5, --cpu=MPCore --arm --apcs=/interwork -O2 -Otime, every
 * instruction matches the original byte-for-byte modulo b/bl branch
 * immediates (link-layout facts). -O3 produces identical output.
 *
 * Verify with:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllDialogCommon 0x8d0 0x2c \
 *       xy-decomp/armcc-matching/matched/DllDialogCommon/0x8d0.c
 *
 * Names are placeholders: the callee at CRO 0x220 is an import veneer
 * whose real symbol is not yet resolved; the struct is the dialog proc
 * object with two pointer fields at +0x58/+0x5c.
 */

typedef struct T {
    unsigned char pad[0x58];
    void *p58;               /* +0x58 */
    void *p5c;               /* +0x5c */
} T;

extern int A(void *a, void *b, int flag);   /* veneer @ CRO 0x220 */

int go(T *t)
{
    A(t->p5c, t->p58, 0);
    return A(t->p5c, t->p58, 1);
}
