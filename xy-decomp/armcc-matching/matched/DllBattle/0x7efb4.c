/*
 * DllBattle @ 0x7efb4, size 0x84 -- script-command handler: reads two float arguments, substitutes defaults (16.0f / 6000.0f) for zero values, and applies them to the sub-object at +496 via two setters.
 * Masked match 33/33. Likely min/max-distance (near/far) style arguments; pool = 0.0f, 16.0f, 6000.0f.
 * Style: trick(redundant-local) -- named zero local z only to make armcc load the 0.0f pool constant before the first stack float (plain `x == 0.0f` swaps the two vldr).
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x7efb4 0x84 xy-decomp/armcc-matching/matched/DllBattle/0x7efb4.c
 */
extern void rd(int, float*, int);
extern void fx(int, float);
extern void fy(int, float);
typedef struct O { char pad[496]; int h; } O;
void fn(int a, O *o) {
    float x, y;
    rd(a, &x, 1);
    rd(a, &y, 1);
    {
        float z = 0.0f;
        if (x == z) x = 16.0f;
        if (y == z) y = 6000.0f;
    }
    fx(o->h, x);
    fy(o->h, y);
}
