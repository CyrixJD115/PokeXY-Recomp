/* DllField function @ CRO file offset 0xdc9f4, size 0x50 -- MATCHED (masked, 20/20).
 * Maps the kind byte at this+4 to an id: 0->12, 1->13, 2->15, 3->14; any other value calls the
 * game's assert/error stub err(0,0,0) and returns 0.
 * Sibling (same shape, constants 16,17,19,18): 0xe1598.
 * Style: natural -- plain switch
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0xdc9f4 0x50 xy-decomp/armcc-matching/matched/DllField/0xdc9f4.c
 */
typedef struct S { char pad[4]; unsigned char m; } S;
extern void err(int, int, int);
int fn(S *s)
{
    switch (s->m) {
    case 0: return 12;
    case 1: return 13;
    case 2: return 15;
    case 3: return 14;
    default: err(0, 0, 0); return 0;
    }
}
