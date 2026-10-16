/*
 * DllField @ 0x11298 (size 0x6c) -- masked match 27/27 (no --mask-ranges; compare-chain switch).
 * Event dispatcher: obj = this->f7c; if obj non-null and k is 0x21 / 0x27 / 0x3c, call
 * obj vtable slot 0 / 1 / 2 (obj) and return 0; every other case (and null obj) returns 1.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x11298 0x6c xy-decomp/armcc-matching/matched/DllField/0x11298.c
 */
typedef struct O { void **vt; } O;
int fn(char *this, int k)
{
    O *o = *(O **)(this + 0x7c);
    if (o) {
        switch (k) {
        case 0x21: ((void (*)(O *))o->vt[0])(o); return 0;
        case 0x27: ((void (*)(O *))o->vt[1])(o); return 0;
        case 0x3c: ((void (*)(O *))o->vt[2])(o); return 0;
        }
    }
    return 1;
}
