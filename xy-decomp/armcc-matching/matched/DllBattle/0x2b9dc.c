/*
 * DllBattle @ 0x2b9dc, size 0x18 -- writes constant pair (-94.0f, -54.0f) to arg2->{x,y}; arg1 unused (default position/offset reset).
 * Masked match 6/6.
 * Style: trick(redundant-local) -- named float locals a,b make armcc pair them into one vstmia instead of two vstr
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x2b9dc 0x18 xy-decomp/armcc-matching/matched/DllBattle/0x2b9dc.c
 */
typedef struct V { float x, y; } V;
void fn(void *o, V *v) { float a = -94.0f, b = -54.0f; v->x = a; v->y = b; }
