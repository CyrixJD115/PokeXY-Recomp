/*
 * DllBattle @ 0x3f444, size 0x28 -- teardown stage: if the owned member at +0x840 is non-null, release it
 * (fnB(fnA(member))), then clear the member pointer.
 * Masked match 10/10.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x3f444 0x28 xy-decomp/armcc-matching/matched/DllBattle/0x3f444.c
 */
typedef struct O { char pad0[2112]; void *a; } O;
extern void *fnA(void *p);
extern void fnB(void *p);
void fn(O *o) {
    if (o->a) {
        fnB(fnA(o->a));
    }
    o->a = 0;
}
