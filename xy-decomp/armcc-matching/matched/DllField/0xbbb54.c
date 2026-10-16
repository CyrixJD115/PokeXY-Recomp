// DllField@0xbbb54, size 0x10, 4/4 masked match.
// Deleting-destructor style thunk: runs the destructor body (0x6d64c, returns
// `this`) then tail-calls the free helper (0x1c8) with that result
// (`push {r4,lr}; bl; pop {r4,lr}; b`).
// Only caller of 0x6d64c in the module; the 0x6d64c body itself is not matched.
// Style: natural -- nested call, outer call in tail position.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xbbb54 0x10 \
//       xy-decomp/armcc-matching/matched/DllField/0xbbb54.c

extern void *Dtor(void *);
extern void Free(void *);
void Fn(void *t) {
    Free(Dtor(t));
}
