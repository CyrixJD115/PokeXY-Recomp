/* DllSalon@0x380 — field-accessor + boolean predicate.
 * Style: natural
 * Body: push {r4,lr}; r0 = r0->f48; r0 += 4; r0 = fn_aa0(r0);
 *        return r0 == 0 ? 1 : 0;  pop {r4,pc}.
 * Internal call to 0xaa0 is masked (link-time fact).
 * Verify: python3 fingerprint_fn.py DllSalon 0x380 0x20 0x380.c
 */
struct S { int pad[0x12]; void *p48; };
struct T { int hdr; int v; };

extern int fn_aa0(struct T *);

int func_380(struct S *this) {
    return fn_aa0((struct T *)((char *)this->p48 + 4)) == 0 ? 1 : 0;
}
