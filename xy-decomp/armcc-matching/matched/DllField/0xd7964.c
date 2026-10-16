// DllField@0xd7964, size 0x2c, 11/11 masked match.
// If this->[+184] (an owned sub-object) is non-null, passes it to an import,
// then forwards (arg, result) to a second import as a tail call. No-op if null.
// Style: natural -- plain null-guarded call chain.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xd7964 0x2c \
//       xy-decomp/armcc-matching/matched/DllField/0xd7964.c

extern int ImportA(int);
extern void ImportB(int, int);

void Fn(char *self, int arg) {
    int o = *(int *)(self + 184);
    if (o != 0) {
        ImportB(arg, ImportA(o));
    }
}
