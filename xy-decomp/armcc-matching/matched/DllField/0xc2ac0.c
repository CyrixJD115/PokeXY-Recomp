// DllField@0xc2ac0, size 0x40, 16/16 masked match.
// Argument-marshalling thunk: tail-forwards seven values to an import -- **(a->[4]), *c, d->[16], b->[8], b->[12], 0, and its own first stack argument e.
// Style: natural -- one forwarding call
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xc2ac0 0x40 \
//       xy-decomp/armcc-matching/matched/DllField/0xc2ac0.c

extern int fa(int, int, int, int, int, int, int);
int Fn(int **a, int *b, int *c, int *d, int e)
{
    return fa(**(int **)((char *)a + 4), *c, d[4], b[2], b[3], 0, e);
}
