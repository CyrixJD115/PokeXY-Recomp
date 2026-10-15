/*
 * DllStatus @ 0xa378 (size 0x88) -- masked match 34/34.
 * Command dispatch: obj=this->f4; for k in {0,1,11,13} call obj vtable slot {0,1,2,3}(obj) and return 1; else 0.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStatus 0xa378 0x88 xy-decomp/armcc-matching/matched/DllStatus/0xa378.c
 */
typedef struct O { void **vt; } O;
int fn(char *this, int k)
{
    O *o = *(O **)(this + 4);
    if (o) {
        switch (k) {
        case 0:  ((void (*)(O *))o->vt[0])(o); return 1;
        case 1:  ((void (*)(O *))o->vt[1])(o); return 1;
        case 11: ((void (*)(O *))o->vt[2])(o); return 1;
        case 13: ((void (*)(O *))o->vt[3])(o); return 1;
        }
    }
    return 0;
}
