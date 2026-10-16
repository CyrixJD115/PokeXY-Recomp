// DllField@0xcc474, size 0x58, 22/22 masked match.
// State-handler step: calls an import on this->[68] (arg 1); if byte +145 is set and the state id at this->[68]+184 is 104, sets bit 0x80000 in flags +168, clears +148 and returns 1; otherwise returns 38.
// Style: natural -- one short-circuit condition, early return 38
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xcc474 0x58 \
//       xy-decomp/armcc-matching/matched/DllField/0xcc474.c

extern void fa(void *, int);
extern int fb(void *);
int Fn(char *s)
{
    fa(*(void **)(s + 68), 1);
    if (*(unsigned char *)(s + 145) == 0 || fb(*(char **)(s + 68) + 184) != 104)
        return 38;
    *(int *)(s + 168) |= 0x80000;
    *(int *)(s + 148) = 0;
    return 1;
}
