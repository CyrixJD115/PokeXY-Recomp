// DllField@0xc2a7c, size 0x44, 17/17 masked match.
// Release helper: if this->[168] is set, passes it through two imports (second consumes the first's result) and zeroes it; if this->[156] is set, virtual-deletes it (vtable slot 1) and zeroes it.
// Style: natural -- two guarded release blocks
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xc2a7c 0x44 \
//       xy-decomp/armcc-matching/matched/DllField/0xc2a7c.c

extern void *fa(void *);
extern void fb(void *);
typedef struct O { void (**vt)(struct O *); } O;
void Fn(char *s)
{
    if (*(void **)(s + 168)) {
        fb(fa(*(void **)(s + 168)));
        *(void **)(s + 168) = 0;
    }
    if (*(O **)(s + 156)) {
        (*(*(O **)(s + 156))->vt[1])(*(O **)(s + 156));
        *(O **)(s + 156) = 0;
    }
}
