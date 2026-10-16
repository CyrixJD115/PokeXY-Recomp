/*
 * DllBattle @ 0x89acc, size 0x58 -- script-command handler: reads an index and a float from the script context, remaps the index through the table at +1744, and passes the float plus the member at +184 of the object at table +280 to a setter.
 * Masked match 22/22. Sibling of 0x7af3c (same index remap).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x89acc 0x58 xy-decomp/armcc-matching/matched/DllBattle/0x89acc.c
 */
extern void rd(int, int*, int);
extern void rf(int, float*, int);
extern void fq(int, float);
typedef struct X { char pad[184]; int m184; } X;
typedef struct O { char pad[280]; X *b[1]; char pad2[1744 - 284]; int a[1]; } O;
void fn(int a, O *o) {
    int i;
    float f;
    rd(a, &i, 1);
    rf(a, &f, 1);
    i = o->a[i];
    fq(o->b[i]->m184, f);
}
