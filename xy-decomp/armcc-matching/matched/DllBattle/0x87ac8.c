/*
 * DllBattle @ 0x87ac8, size 0x3c -- script-command handler: reads one int argument and stores (uint16)v * pi/180 (degrees to radians) into the float at +496 of the sub-object at +684.
 * Masked match 15/15. Constant 0.017453292f lives in the literal pool.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x87ac8 0x3c xy-decomp/armcc-matching/matched/DllBattle/0x87ac8.c
 */
extern void rd(int, int*, int);
typedef struct Q { char pad[496]; float deg; } Q;
typedef struct O { char pad[684]; Q *q; } O;
void fn(int a, O *o) { int v; rd(a, &v, 1); o->q->deg = (float)(unsigned short)v * 0.017453292f; }
