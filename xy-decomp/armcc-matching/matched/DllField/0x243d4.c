// DllField@0x243d4, size 0x2c (0x28 code + 1 vtable pool word), 10/10 masked match.
// Constructor: calls the base ctor (import veneer, returns this), then clears
// ints at +36/+40/+44 and byte +48, installs the class vtable (pool word) at
// +0, returns this. Sibling shape of 0x1980c.
//
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x243d4 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x243d4.c

extern char VT[];
extern char *Base(void *);

char *Fn(void *this) {
    char *p = Base(this);
    *(int *)(p + 36) = 0;
    *(int *)p = (int)VT;
    *(int *)(p + 40) = 0;
    *(int *)(p + 44) = 0;
    p[48] = 0;
    return p;
}
