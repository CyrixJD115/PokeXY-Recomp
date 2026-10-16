// DllField@0x334d8, size 0x20, 8/8 masked match.
// Destructor stage: calls an import veneer (this, 1), then another on the sub-object at this+108, returns this.
// Style: natural -- plain call sequence
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x334d8 0x20 \
//       xy-decomp/armcc-matching/matched/DllField/0x334d8.c

extern void V(void *, int);
extern void W(void *);
void *Fn(char *this) { V(this, 1); W(this + 108); return this; }
