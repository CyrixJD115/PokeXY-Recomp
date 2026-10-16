/*
 * DllBattle @ 0x2c60, size 0x30 -- if (this->+0x24) call its vtbl slot 1 (delete); free this->+0x20 via import; return 1.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x2c60 0x30 xy-decomp/armcc-matching/matched/DllBattle/0x2c60.c
 */
typedef struct O { void **vt; } O;
extern void fn_free(void *);
int fn(char *self) {
    O *o = *(O **)(self + 0x24);
    if (o != 0) ((void (*)(O *))o->vt[1])(o);
    fn_free(*(void **)(self + 0x20));
    return 1;
}
