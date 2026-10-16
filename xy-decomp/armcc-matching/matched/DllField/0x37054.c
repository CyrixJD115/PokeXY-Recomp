// DllField@0x37054, size 0x2c (0x28 code + 1 vtable pool word), 11/11 masked match.
// Constructor: calls the base ctor (import veneer, returns this), installs the
// class vtable (pool word), clears bytes +24/+52/+60 and the int at +68,
// returns this.
//
// Style: trick(redundant-local) -- `char z = 0` shared by every store only to make armcc
// load the vtable pool word into r2 and the zero into r1 in retail's order
// (literal zeros in the same order schedule the ldr into r1 instead).
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x37054 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x37054.c

extern char VT[];
extern char *Base(void *);

char *Fn(void *this) {
    char *p = Base(this);
    char z = 0;
    *(int *)p = (int)VT;
    p[24] = z;
    p[52] = z;
    p[60] = z;
    *(int *)(p + 68) = z;
    return p;
}
