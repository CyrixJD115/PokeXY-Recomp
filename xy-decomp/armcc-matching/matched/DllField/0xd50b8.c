// DllField@0xd50b8, size 0x34 (incl. 1-word 0.0f literal), 13/13 masked match.
// Float getter: looks up an object via Import(this->[+12]); if that is null
// returns 0.0f, otherwise returns Import2(Import(this->[+12])) (tail call;
// the lookup is simply called twice, as written).
// Style: natural -- plain null check then forward.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xd50b8 0x34 \
//       xy-decomp/armcc-matching/matched/DllField/0xd50b8.c

extern int Lookup(int);
extern float GetValue(int);

float Fn(char *self) {
    if (Lookup(*(int *)(self + 12)) == 0)
        return 0.0f;
    return GetValue(Lookup(*(int *)(self + 12)));
}
