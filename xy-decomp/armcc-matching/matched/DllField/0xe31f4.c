// DllField@0xe31f4, size 0x30, 12/12 masked match.
// State-handler step: o = arg->[0]; if o's flag byte (+0x87f) is set and an
// import check on (o, 0x800000) succeeds, returns 1; otherwise returns 2.
// Style: natural -- plain `if (a && b) return 1; return 2;`.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xe31f4 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0xe31f4.c

extern int Check(char *, int);

int Fn(void *self, char **a) {
    char *o = a[0];
    if (o[2175] != 0 && Check(o, 0x800000) != 0)
        return 1;
    return 2;
}
