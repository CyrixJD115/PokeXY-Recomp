// DllField@0xca54c, size 0x30, 12/12 masked match.
// Calls two imports (second returns a flag); if the flag is 0 and the pointer at this+68 is non-null, tail-calls an import on that pointer, else returns.
// Identical bytes to 0xc9060.
// Style: natural -- plain && condition and guarded tail call
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xca54c 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0xca54c.c

extern int fa(void);
extern int fb(void);
extern void fc(void *);
struct S { char pad[68]; void *p; };
void Fn(struct S *s)
{
    fa();
    if (fb() == 0 && s->p != 0)
        fc(s->p);
}
