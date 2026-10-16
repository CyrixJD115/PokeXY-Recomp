// DllField@0xe248c, size 0x24, 9/9 masked match.
// Event/handler thunk: takes the object at arg->[+4], calls its vtable slot
// 55 (+0xdc) with (obj, 0) and returns whether the result is non-zero.
// (Enumerated size 0x2c in the task list was over-wide: the trailing
// `mov r0,#12; bx lr` at 0xe24b0 is a separate function.)
// Style: natural -- plain virtual call, result != 0.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xe248c 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0xe248c.c

struct O { int (**vt)(struct O *, int); };

int Fn(void *self, int *a) {
    struct O *o = (struct O *)a[1];
    return o->vt[55](o, 0) != 0;
}
