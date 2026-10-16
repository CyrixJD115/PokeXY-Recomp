// DllField@0xca1a8, size 0x24, 9/9 masked match.
// Setter: byte [+154] = v; when v is 0, first calls an import on this->[+68] if non-null.
// Style: natural -- plain if/store (int arg gives the retail arg-move order)
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xca1a8 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0xca1a8.c

extern void H(void *);
void Fn(char *t, int v) {
    if (!v && *(int *)(t + 68) != 0) H(*(void **)(t + 68));
    t[154] = v;
}
