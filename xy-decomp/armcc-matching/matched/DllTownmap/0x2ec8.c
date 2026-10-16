/* DllTownmap function @ CRO file offset 0x2ec8, size 0x24 — MATCHED (masked, 9/9).
 * Calls fn(this->[0x5c], this->[0x58], 0), sets flag byte @0xc1 = 1, returns 1
 * (the stored 1 doubles as the return value).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllTownmap 0x2ec8 0x24 xy-decomp/armcc-matching/matched/DllTownmap/0x2ec8.c
 */
extern void fn(void *a, int b, int c);
struct T { char pad[0x58]; int a; void *m; char pad3[0xc1 - 0x60]; unsigned char flag; };
int f(struct T *s)
{
    fn(s->m, s->a, 0);
    s->flag = 1;
    return 1;
}
