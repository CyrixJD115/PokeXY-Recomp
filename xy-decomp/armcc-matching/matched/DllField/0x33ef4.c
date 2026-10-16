// DllField@0x33ef4, size 0x2c, 11/11 masked match.
// Release-owned-object helper: if the pointer member at this+48 is non-null,
// call its virtual method in vtable slot 1 (+0x4), then clear the
// member. Same shape at DllField 0x2b380, 0x31b40, 0x33ef4, 0x36bfc, 0x387d8
// (slot 1) and 0x3f034 (slot 6) with different member offsets.
//
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x33ef4 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x33ef4.c

typedef struct O { void **vt; } O;
typedef struct T { char pad[48]; O *p; } T;

void Fn(T *this) {
    O *o = this->p;
    if (o) {
        ((void (*)(O *))o->vt[1])(o);
        this->p = 0;
    }
}
