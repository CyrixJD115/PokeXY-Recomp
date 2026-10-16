// DllField@0xb1a10, size 0x50 (incl. the trailing vtable pool word), 20/20 masked match.
// Destructor stage: installs the class vtable, virtual-deletes (vtable slot 1) the member at +0x430 if set and zeroes it, frees the handle at +4 via an import if set and zeroes it, returns this.
// Style: natural -- vtable store first, then two guarded member releases
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xb1a10 0x50 \
//       xy-decomp/armcc-matching/matched/DllField/0xb1a10.c

extern void *VT[];
extern void fa(void *);
typedef struct O { void (**vt)(struct O *); } O;
char *Fn(char *s)
{
    *(void ***)s = VT;
    if (*(O **)(s + 1072)) {
        (*(*(O **)(s + 1072))->vt[1])(*(O **)(s + 1072));
        *(O **)(s + 1072) = 0;
    }
    if (*(void **)(s + 4)) {
        fa(*(void **)(s + 4));
        *(void **)(s + 4) = 0;
    }
    return s;
}
