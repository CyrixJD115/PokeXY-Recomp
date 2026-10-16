/*
 * DllBattle @ 0x4d348, size 0x1c -- looks up an object via this->[2], returns its byte field at +25, or 31 if none.
 * Masked match 7/7.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x4d348 0x1c xy-decomp/armcc-matching/matched/DllBattle/0x4d348.c
 */
extern char *g(void *);
int fn(int *o) { char *r = g((void *)o[2]); if (r) return (unsigned char)r[25]; return 31; }
