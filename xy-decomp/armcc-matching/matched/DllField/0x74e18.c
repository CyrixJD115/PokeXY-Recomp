/* DllField function @ CRO file offset 0x74e18, size 0x10 -- MATCHED (masked, 4/4).
 * Sets this->kind (+134) = 4 and returns the address of the sub-record at arg1+332.
 * Family: `mov r2,#K; strb r2,[r0,#134]; add r0,r1,#OFF; bx lr`; siblings 0x74dd8 (K=2,OFF=276),
 * 0x74de8 (3,296), 0x74df8 (3,308), 0x74e08 (3,320), 0x74e18 (4,332).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x74e18 0x10 xy-decomp/armcc-matching/matched/DllField/0x74e18.c
 */

struct S { char pad[134]; char kind; };
struct B { char pad[332]; char sub[20]; };

char *sel(struct S *s, struct B *b)
{
    s->kind = 4;
    return b->sub;
}
