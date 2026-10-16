// DllField@0x1bd34, size 0x14, 5/5 masked match.
// Call+return helper: calls an import veneer with this, returns this.
// Style: natural -- plain call then return this
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x1bd34 0x14 \
//       xy-decomp/armcc-matching/matched/DllField/0x1bd34.c

extern void V(void *);
void *Fn(void *this) { V(this); return this; }
