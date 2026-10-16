/*
 * DllBattle @ 0xc9c98, size 0x14 -- deleting-dtor thunk for a secondary base: this-=0xe8, call the base dtor, then tail-call delete on the returned pointer.
 * Masked match.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xc9c98 0x14 xy-decomp/armcc-matching/matched/DllBattle/0xc9c98.c
 */
extern void *dtor(void *);
extern void del(void *);
void fn(char *p) { del(dtor(p - 0xe8)); }
