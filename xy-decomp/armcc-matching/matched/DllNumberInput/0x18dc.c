/* DllNumberInput function @ CRO file offset 0x18dc, size 0x20 — MATCHED (masked, 8/8). Copies a field from the second argument into a sub-object, calls a veneer with it, stashes the result, returns 1.
 * Style: natural
 *
 * `this->n24` (member at +0x24) is a pointer to a small node object with
 * fields at +4 and +8; `v->f14` (member at +0x14 of the second argument)
 * is stored into node+4 and also passed to an unresolved import veneer
 * whose return lands in node+8.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllNumberInput 0x18dc 0x20 xy-decomp/armcc-matching/matched/DllNumberInput/0x18dc.c
 */

typedef struct V { unsigned char pad[20]; void *f14; } V;
typedef struct N { unsigned char pad[4]; void *f4; void *f8; } N;
typedef struct T2 { unsigned char pad[36]; N *n24; } T2;

extern void *fn398(void *a);

int F(T2 *t, V *v)
{
    N *n;
    void *tmp;
    n = t->n24;
    tmp = v->f14;
    n->f4 = tmp;
    n->f8 = fn398(tmp);
    return 1;
}
