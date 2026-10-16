// DllField@0x1acd4, size 0x28, 10/10 masked match.
// Setter-with-notify: loads the child object at this+0x2c, calls its virtual
// method at vtable slot 16 (+0x40) with (child, v), then stores v as a byte
// at this+200.
//
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x1acd4 0x28 \
//       xy-decomp/armcc-matching/matched/DllField/0x1acd4.c

typedef struct O O;
struct O { void **vt; };

void Fn(char *this, int v) {
    O *o = *(O **)(this + 44);
    ((void (*)(O *, int))o->vt[16])(o, v);
    this[200] = v;
}
