/*
 * DllBattle @ 0x2ef0, size 0x44 -- if (this->+0xc) { vcall slot 4; if still set vcall slot 1; this->+0xc = 0 }.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x2ef0 0x44 xy-decomp/armcc-matching/matched/DllBattle/0x2ef0.c
 */
typedef struct O { void **vt; } O;
void fn(char *self) {
    O *o = *(O **)(self + 0xc);
    if (o != 0) {
        ((void (*)(O *))o->vt[4])(o);
        o = *(O **)(self + 0xc);
        if (o != 0) ((void (*)(O *))o->vt[1])(o);
        *(int *)(self + 0xc) = 0;
    }
}
