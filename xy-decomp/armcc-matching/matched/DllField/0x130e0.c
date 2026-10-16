// DllField@0x130e0, size 0x2c, 11/11 masked match.
// Event/notify handler: if the child object at this+0x7c exists and the
// event id (2nd arg) is 20, call the child's virtual method at vtable slot 0;
// always returns 1. (Arg name/semantics inferred, not verified.)
//
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x130e0 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0x130e0.c

typedef struct O { void **vt; } O;
typedef struct T { char pad[124]; O *p; } T;

int Fn(T *this, int id) {
    O *o = this->p;
    if (o && id == 20) {
        ((void (*)(O *))o->vt[0])(o);
    }
    return 1;
}
