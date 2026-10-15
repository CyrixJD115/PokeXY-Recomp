/*
 * DllBattle @ 0x42c30, size 0x24 -- vt[0x80](this,a) result compared: returns r==103||r==104.
 * Masked match 9/9. The forwarded int arg (r1 passthrough) is what moves the vtable load into r2.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x42c30 0x24 xy-decomp/armcc-matching/matched/DllBattle/0x42c30.c
 */
typedef struct O { int (**vt)(struct O *, int); } O;
int fn(O *o, int a) { int r = o->vt[32](o, a); return r == 103 || r == 104; }
