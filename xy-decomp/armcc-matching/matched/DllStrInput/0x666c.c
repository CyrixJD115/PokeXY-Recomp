/* DllStrInput function @ CRO file offset 0x666c, size 0x8 — MATCHED (masked, 2/2). Trivial byte-field setter: `this->field_0xd9 = v;`, void return.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStrInput 0x666c 0x8 xy-decomp/armcc-matching/matched/DllStrInput/0x666c.c
 */

typedef struct T { unsigned char pad[217]; unsigned char flag; } T;

void F(T *t, int v)
{
    t->flag = (unsigned char)v;
}
