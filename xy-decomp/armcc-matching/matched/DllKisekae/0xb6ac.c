/* DllKisekae function @ CRO file offset 0xb6ac, size 0x34 (incl. trailing pool word) -- MASKED MATCH (13/13).
 * Destructor-stage helper: loads the owned member at +0x14, installs the
 * vtable pointer, frees the member if non-null (masked call) and zeroes it,
 * returns `this`. Member load precedes the vtable pool load here (the
 * opposite of 0xb674).
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllKisekae 0xb6ac 0x34 xy-decomp/armcc-matching/matched/DllKisekae/0xb6ac.c
 */
extern char VT[];
extern void fnFree(void *);
typedef struct { void *vptr; char pad[16]; void *m; } O;

O *dtor(O *o)
{
    void *m = o->m;
    o->vptr = VT;
    if (m) { fnFree(m); o->m = 0; }
    return o;
}
