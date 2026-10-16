/*
 * DllBattle @ 0x2f34, size 0x40 -- lazy-member creator: n = new(12, this->@0x1c); if (n) { n->@4 = n->@8 = 0; n->vt = <vtable> } this->@0x10 = n; n->@4 = this (back-pointer). Pool word is a relocation. Sibling of 0x2984.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x2f34 0x40 xy-decomp/armcc-matching/matched/DllBattle/0x2f34.c
 */
extern void *fn_new(int sz, int a);
extern int vtbl_x[];
typedef struct N { int *vt; void *owner; int b; } N;
void fn(char *self) {
    N *n = (N *)fn_new(12, *(int *)(self + 0x1c));
    if (n) { n->owner = 0; n->b = 0; n->vt = vtbl_x; }
    *(N **)(self + 0x10) = n;
    n->owner = self;
}
