// DllField@0xc6b40, size 0x10, 4/4 masked match.
// Deleting-destructor thunk: calls the local destructor at 0xc6b50 then tail-calls operator delete (0x1c8) on the returned pointer.
// Style: natural -- del(D(t))
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xc6b40 0x10 \
//       xy-decomp/armcc-matching/matched/DllField/0xc6b40.c

extern void *D(void *);
extern void del(void *);
void Fn(void *t) { del(D(t)); }
