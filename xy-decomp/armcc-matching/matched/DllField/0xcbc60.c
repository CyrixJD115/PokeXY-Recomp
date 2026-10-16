// DllField@0xcbc60, size 0x3c, 15/15 masked match.
// State-handler step: calls an import on this->[68] (arg 1), then reads the state id of the object at this->[68]+184; if not 103 returns 36, else calls an import on this and returns 1.
// Style: natural -- early return then call-and-return-1
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xcbc60 0x3c \
//       xy-decomp/armcc-matching/matched/DllField/0xcbc60.c

extern void fa(void *, int);
extern int fb(void *);
extern void fc(void *);
int Fn(char *s)
{
    fa(*(void **)(s + 68), 1);
    if (fb(*(char **)(s + 68) + 184) != 103)
        return 36;
    fc(s);
    return 1;
}
