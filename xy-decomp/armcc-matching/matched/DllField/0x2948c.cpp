/* DllField @ 0x2948c size 0x44 MATCHED (masked 17/17): guarded release of a sub-object's pending state.
 * `t = this->t@+4; if (t) { if (t->x@+4) { fa(arg, t->x); t->st@+32 = -1; } else fb(0,0,0) (tail call) }`
 * Style: trick(cpp-inline-member) -- the body only matches when written as an inline C++ member of the pointee
 *   (`U::Go()`, called on the loaded pointer). Plain-C form of the same logic is 5/17: armcc if-converts the
 *   `t->x` test arm and drops the tail-call/early-return shape; as an inline member `this` stays live in a
 *   callee-saved register and armcc keeps retail's branch layout (see matched/DllField/0xb1bb8.cpp for the
 *   mechanism). Needs --cpp (.cpp suffix).
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x2948c 0x44 xy-decomp/armcc-matching/matched/DllField/0x2948c.cpp
 */
struct T { int pad0; void *x; char pad[24]; int st; };
struct S { int pad0; T *t; };
extern void fa(void *a, void *b);
extern void fb(int a, int b, int c);
struct U : T {
    void Go(void *a) { if (x) { fa(a, x); st = -1; } else fb(0, 0, 0); }
};
void f(S *s, void *a) { if (s->t) ((U *)s->t)->Go(a); }
