/*
 * DllBattle @ 0x393d8, size 0x20 -- virtual call returning object, bool of its byte +5: return this->vt[0x3e](this)->b5 != 0.
 * Masked match 8/8.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x393d8 0x20 xy-decomp/armcc-matching/matched/DllBattle/0x393d8.c
 */
typedef struct P { char pad[5]; unsigned char b; } P;
typedef struct O { P *(**vt)(struct O *); } O;
int fn(O *o) {
    return o->vt[62](o)->b != 0;
}
