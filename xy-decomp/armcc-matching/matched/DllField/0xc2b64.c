// DllField@0xc2b64, size 0x20, 8/8 masked match.
// Bool getter: if this->[+152] (a handle/pointer) is non-null, returns
// whether an import call on it is non-zero; null yields 0 (the loaded value).
// Style: natural -- reuse the loaded field as the result variable.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xc2b64 0x20 \
//       xy-decomp/armcc-matching/matched/DllField/0xc2b64.c

extern int ImportA(void);

int Fn(char *this) {
    int v = *(int *)(this + 152);
    if (v != 0) {
        v = ImportA() != 0;
    }
    return v;
}
