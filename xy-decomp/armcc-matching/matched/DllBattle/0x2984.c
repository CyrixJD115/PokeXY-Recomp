/*
 * DllBattle @ 0x2984, size 0x40 -- lazy-member creator: n = new(12, this->@0x20); if (n) { n->@4 = n->@8 = 0; n->vt = <vtable> } this->@0x14 = n; n->@4 = this (back-pointer). Pool word is a relocation. Masked match 16/16.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x2984 0x40 xy-decomp/armcc-matching/matched/DllBattle/0x2984.c
 */
extern void *fn_new(int sz, int a);
extern int vtbl_x[];
typedef struct N { int *vt; void *owner; int b; } N;
void fn(char *self) {
    N *n = (N *)fn_new(12, *(int *)(self + 0x20));
    if (n) { n->owner = 0; n->b = 0; n->vt = vtbl_x; }
    *(N **)(self + 0x14) = n;
    n->owner = self;
}
