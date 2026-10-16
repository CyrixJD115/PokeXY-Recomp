// DllField@0xe1208, size 0x1c, 7/7 masked match.
// Calls one void import, then a second import and returns whether its
// result equals 3 (bool getter: "state == 3" after a refresh call).
// Style: natural -- plain call, call, compare-and-return-bool.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xe1208 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0xe1208.c

extern void ImportA(void);
extern int ImportB(void);

int Fn(void) {
    ImportA();
    return ImportB() == 3;
}
