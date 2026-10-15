/*
 * DllPuzzle @ 0x99dc (size 0x24) -- masked match 9/9.
 * Copies four ints from a source struct (s[0..3]) into this+0x3c, +0x40, +0xb0, +0xb4.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllPuzzle 0x99dc 0x24 xy-decomp/armcc-matching/matched/DllPuzzle/0x99dc.c
 */
typedef struct O { unsigned char p[60]; int f3c, f40; unsigned char q[108]; int fb0, fb4; } O;
void F(O *t, int *s){ t->f3c = s[0]; t->f40 = s[1]; t->fb0 = s[2]; t->fb4 = s[3]; }
