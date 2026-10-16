// DllField@0xcaf30, size 0x5c, 23/23 masked match.
// State-handler: A(this->[+68],1); switches on Cur(sub@+184): 40 -> Z(sub,1), returns 2; 41 -> Go(this), returns 1; else returns 12.
// Style: natural -- plain switch
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xcaf30 0x5c \
//       xy-decomp/armcc-matching/matched/DllField/0xcaf30.c

extern void A(void *, int);
extern int Cur(void *);
extern void Go(void *);
extern void Z(void *, int);
int Fn(char *t) {
    A(*(void **)(t + 68), 1);
    switch (Cur(*(char **)(t + 68) + 184)) {
    case 40:
        Z(*(char **)(t + 68) + 184, 1);
        return 2;
    case 41:
        Go(t);
        return 1;
    default:
        return 12;
    }
}
