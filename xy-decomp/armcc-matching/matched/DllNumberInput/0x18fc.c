/* DllNumberInput function @ CRO file offset 0x18fc, size 0x78 — MATCHED (masked, 30/30). Constructor: fetches an object, installs a vtable, builds a buffer via an import veneer, placement-allocates a second node using a moving pointer cursor.
 *
 * Reverse-engineering note: the node-initialization tail is written as a
 * SEQUENCE OF CURSOR MOVES (`str r6,[r0,#92]!` / `add r0,r0,#4` / `str
 * r6,[r0,#-84]!` / `str r5,[r0,#-12]!` — each a pre-indexed store that
 * both writes AND advances the pointer) rather than a set of fixed-offset
 * accesses off one stable base pointer. Translating it the same way in C
 * (`p += 0x5c; *p = 0; p += 4; p = fn(p); p -= 0x54; *p = 0; p -= 0xc;
 * *p = parent;`) reproduced the retail bytes exactly on the first try.
 * This is the sibling pattern to `DllDialogCommon@0x710` (same shape:
 * fetch object, install vtable, build buffer, placement-allocate a node)
 * — but that specimen's node tail uses one `sub`-then-fixed-offsets
 * instead of a continued cursor chain, and still doesn't match; see
 * `docs/decomp/armcc-toolchain.md`'s open-item note on `0x710`.
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllNumberInput 0x18fc 0x78 xy-decomp/armcc-matching/matched/DllNumberInput/0x18fc.c
 */

typedef struct O2 { void *vptr; unsigned char pad1[0x1c]; void *field20; void *field24; } O2;

extern void *VT[];
extern void *fn3a0(void *a);
extern void *fn3a8(int code);
extern void *fn3b0(void *h, int a, int b, int c, int d);
extern void *fn218(int size, void *ctx);
extern void *fn3b8(void *p);

void *F(void *arg0, void *parent)
{
    O2 *o;
    void *h;
    void *buf;
    void *mem;
    unsigned char *p;

    o = (O2 *)fn3a0(arg0);
    o->vptr = VT;
    h = fn3a8(22);
    buf = fn3b0(h, 0x1a3, 0x1000, 0, 0);
    o->field20 = buf;

    mem = fn218(0x78, buf);
    if (mem) {
        p = (unsigned char *)mem;
        p += 0x5c;
        *(int *)p = 0;
        p += 4;
        p = (unsigned char *)fn3b8(p);
        p -= 0x54;
        *(int *)p = 0;
        p -= 0xc;
        *(void **)p = parent;
        mem = p;
    }
    o->field24 = mem;
    return o;
}
