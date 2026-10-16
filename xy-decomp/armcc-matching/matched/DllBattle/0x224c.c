/*
 * DllBattle @ 0x224c, size 0x30 -- teardown step: if (this->@0x24) vcall slot 1 (release); then fn(this->@0x20); return 1.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x224c 0x30 xy-decomp/armcc-matching/matched/DllBattle/0x224c.c
 */
extern void fn_rel(void *p);
typedef struct O { void **vt; } O;
int fn(char *self) {
    O *o = *(O **)(self + 0x24);
    if (o) ((void (*)(O *))o->vt[1])(o);
    fn_rel(*(void **)(self + 0x20));
    return 1;
}
