// DllField@0x1980c, size 0x28 (0x24 code + 1 vtable pool word), 10/10 masked match.
// Constructor: calls the base ctor (import veneer, returns this), installs this
// class's vtable (pool word, relocated at link), clears three int fields at
// +24/+28/+32, returns this.
//
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x1980c 0x28 \
//       xy-decomp/armcc-matching/matched/DllField/0x1980c.c

extern char VT[];
extern int *Base(void *);

int *Fn(void *this) {
    int *p = Base(this);
    p[6] = 0;
    p[0] = (int)VT;
    p[7] = 0;
    p[8] = 0;
    return p;
}
