// DllField@0xc8f88, size 0x3c, 15/15 masked match.
// State-handler step: if an import on (this->[100], this->[180]) returns 0 returns 50; else calls a second import with the same pair, sets state this->[180] = 13 and returns the byte at this->[189] (read before the state store).
// Style: trick(ordering-local) -- the byte read must be a local assigned before the state store, else armcc schedules the ldrb after the str (12/15)
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xc8f88 0x3c \
//       xy-decomp/armcc-matching/matched/DllField/0xc8f88.c

extern int fa(void *, int);
extern void fb(void *, int);
int Fn(char *s)
{
    int r;
    if (fa(*(void **)(s + 100), *(int *)(s + 180)) == 0)
        return 50;
    fb(*(void **)(s + 100), *(int *)(s + 180));
    r = *(unsigned char *)(s + 189);
    *(int *)(s + 180) = 13;
    return r;
}
