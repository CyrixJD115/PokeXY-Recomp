/*
 * DllBattle @ 0x82cb4, size 0x64 -- script-command handler: reads a vec3 and an int mode from the script context; if mode == 1 and the flag byte at +605 is set it negates the vector's z component, then passes the vector to a setter on the sub-object at +200.
 * Masked match 25/25. Sibling 0x8711c has the same tail but initialises the vec3 from three pool constants (0.2, -0.45, -0.55) instead of reading it; that init (vstmia from s0-s2 via r0=sp) did not reproduce, see attempted note.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x82cb4 0x64 xy-decomp/armcc-matching/matched/DllBattle/0x82cb4.c
 */
extern void rdv(int, float*, int);
extern void rdi(int, int*, int);
extern void fz(int, float*);
typedef struct O { char pad[200]; int h; char pad2[605 - 204]; unsigned char c; } O;
void fn(int a, O *o) {
    float v[3];
    int m;
    rdv(a, v, 1);
    rdi(a, &m, 1);
    if (m == 1 && o->c) v[2] = -v[2];
    fz(o->h, v);
}
