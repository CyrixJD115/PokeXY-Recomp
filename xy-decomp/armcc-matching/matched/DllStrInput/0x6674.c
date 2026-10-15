/* DllStrInput function @ CRO file offset 0x6674, size 0x14 — MATCHED (masked, 5/5). Boolean getter: `return this->p28->flag16 != 0;`.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStrInput 0x6674 0x14 xy-decomp/armcc-matching/matched/DllStrInput/0x6674.c
 */

typedef struct Inner { unsigned char pad[16]; unsigned char flag16; } Inner;
typedef struct T { unsigned char pad[40]; Inner *p28; } T;

int F(T *t)
{
    return t->p28->flag16 != 0;
}
