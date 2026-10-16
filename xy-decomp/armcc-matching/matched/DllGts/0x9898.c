/*
 * Style: natural
 * DllGts @ 0x9898, size 0x1c, masked match 7/7.
 * If this->obj@0x20 non-null, tail-calls its vtable slot 0x24 (slot 9) with obj as this.
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllGts 0x9898 0x1c xy-decomp/armcc-matching/matched/DllGts/0x9898.c
 */

typedef struct O { unsigned char p[0x20]; void ***f20; } O;
void F(O *t){ void ***o = t->f20; if (o) ((void(*)(void*))(*o)[9])(o); }
