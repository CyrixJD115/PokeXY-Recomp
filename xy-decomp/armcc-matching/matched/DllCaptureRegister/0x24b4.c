/* DllCaptureRegister function @ CRO file offset 0x24b4, size 0x2c -- MATCHED (masked).
 * Calls the same import veneer twice with the object's two fields (r0=f[+0x5c], r1=f[+0x58]) and
 * a flag 1, then again with 0 (tail call). The second call uses a pre-indexed ldr
 * (`ldr r1,[r4,#88]!`), reproduced by the same field loads.
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllCaptureRegister 0x24b4 0x2c xy-decomp/armcc-matching/matched/DllCaptureRegister/0x24b4.c
 */
extern void fn518(int, int, int);
typedef struct O { unsigned char pad[0x58]; int a; int b; } O;

void F(O *t)
{
    fn518(t->b, t->a, 1);
    fn518(t->b, t->a, 0);
}
