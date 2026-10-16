/*
 * DllBattle @ 0x1fd8, size 0xc -- masked match 3/3.
 *
 * Field-getter-plus-offset: loads a pointer member at `this+2960`
 * (0xb90) and returns it offset by a fixed +48 bytes -- same idiom as
 * the sibling accessor at 0x1fc4, but through one pointer indirection
 * instead of a direct `this`-offset.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle \
 *     0x1fd8 0xc xy-decomp/armcc-matching/matched/DllBattle/0x1fd8.c
 */

void *fn(void *o) {
    return *(char **)((char *)o + 2960) + 48;
}
