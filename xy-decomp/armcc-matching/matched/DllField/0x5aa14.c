// DllField@0x5aa14, size 0x1c, 7/7 masked match.
// Getter: G(a)->+12->+28 chain, then reads the u16 at +292 (sub-struct at +288, field at +4).
// Needs a sub-struct (int; u16) at +288: a flat +292 offset makes armcc split the add differently (add #256 + ldrh #36).
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5aa14 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x5aa14.c

extern char *G(void *);
struct T { int a; unsigned short b; };
struct S { char pad[288]; struct T t; };
unsigned short Fn(void *a) {
    struct S *p = *(struct S **)(*(char **)(G(a) + 12) + 28);
    struct T *t = &p->t;
    return t->b;
}
