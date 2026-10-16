// DllField@0xb0378, size 0x3c, 15/15 masked match.
// Boolean query: returns 1 if an import on this returns nonzero, else 1 if the sub-object at this->[0x5a4] is non-null and a second import on it returns nonzero; otherwise 0.
// Style: natural -- plain early-return chain
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xb0378 0x3c \
//       xy-decomp/armcc-matching/matched/DllField/0xb0378.c

extern int fa(void *);
extern int fb(void *);
int Fn(char *s)
{
    if (fa(s) != 0)
        return 1;
    if (*(void **)(s + 1444) != 0 && fb(*(void **)(s + 1444)) != 0)
        return 1;
    return 0;
}
