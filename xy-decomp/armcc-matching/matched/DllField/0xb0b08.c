// DllField@0xb0b08, size 0x10, 4/4 masked match.
// Deleting-destructor thunk: runs a destructor on this (import call) then tail-calls operator delete (0x1c8) on the returned pointer.
// Style: natural -- del(D(t))
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xb0b08 0x10 \
//       xy-decomp/armcc-matching/matched/DllField/0xb0b08.c

extern void *D(void *);
extern void del(void *);
void Fn(void *t) { del(D(t)); }
