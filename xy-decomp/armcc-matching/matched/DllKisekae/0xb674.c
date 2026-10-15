/* DllKisekae function @ CRO file offset 0xb674, size 0x38 -- MASKED MATCH (14/14).
 * Destructor-stage helper: installs the vtable pointer, frees the owned
 * member at +0x14 (if non-null, via a masked call) and zeroes it, then
 * tail-calls the base-class destructor with `this`.
 * Style: trick(ordering-local) -- `void *vt = VT;` and the hoisted member load
 *   exist only to fix armcc's pool-load / member-load order (vt first, then member).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllKisekae 0xb674 0x38 xy-decomp/armcc-matching/matched/DllKisekae/0xb674.c
 */
extern char VT[];
extern void fnFree(void *);
extern void *fnBase(void *);
typedef struct { void *vptr; char pad[16]; void *m; } O;

void *dtor(O *o)
{
    void *vt = VT;
    void *m = o->m;
    o->vptr = vt;
    if (m) { fnFree(m); o->m = 0; }
    return fnBase(o);
}
