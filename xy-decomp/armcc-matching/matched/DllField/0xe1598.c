/* DllField function @ CRO file offset 0xe1598, size 0x50 -- MATCHED (masked, 20/20).
 * Maps the kind byte at this+4 to an id: 0->16, 1->17, 2->19, 3->18; any other value calls the
 * game's assert/error stub err(0,0,0) and returns 0.
 * Sibling (same shape, constants 12,13,15,14): 0xdc9f4.
 * Style: natural -- plain switch
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0xe1598 0x50 xy-decomp/armcc-matching/matched/DllField/0xe1598.c
 */
typedef struct S { char pad[4]; unsigned char m; } S;
extern void err(int, int, int);
int fn(S *s)
{
    switch (s->m) {
    case 0: return 16;
    case 1: return 17;
    case 2: return 19;
    case 3: return 18;
    default: err(0, 0, 0); return 0;
    }
}
