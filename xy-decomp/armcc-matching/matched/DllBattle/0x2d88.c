/*
 * DllBattle @ 0x2d88, size 0x18 -- p = import(); p[0] = <vtable ptr>; (pool word is a relocation, 0 on disk).
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x2d88 0x18 xy-decomp/armcc-matching/matched/DllBattle/0x2d88.c
 */
extern int *fn_get(void);
extern int vtbl_x[];
void fn(void) {
    int *p = fn_get();
    *p = (int)vtbl_x;
}
