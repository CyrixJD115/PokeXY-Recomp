/*
 * DllBattle @ 0x7af3c, size 0x64 -- script-command handler: reads an index and a flag from the script context, remaps the index through the table at +1744, then calls virtual slot 32 (+0x80) on the object at table +280 with (flag != 0).
 * Masked match 25/25. Index is overwritten in place (i = o->a[i]) before the second lookup; the reassigned address-taken local is why retail stores it back to the stack.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x7af3c 0x64 xy-decomp/armcc-matching/matched/DllBattle/0x7af3c.c
 */
extern void rd(int, int*, int);
struct X;
typedef struct X { int (**vt)(struct X*, int); } X;
typedef struct O { char pad[280]; X *b[1]; char pad2[1744 - 284]; int a[1]; } O;
void fn(int a, O *o) {
    int i, f;
    X *x;
    rd(a, &i, 1);
    rd(a, &f, 1);
    i = o->a[i];
    x = o->b[i];
    x->vt[32](x, f != 0);
}
