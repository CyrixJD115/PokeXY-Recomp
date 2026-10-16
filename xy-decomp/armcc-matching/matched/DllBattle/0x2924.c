/*
 * DllBattle @ 0x2924, size 0x60 -- two-owned-object release: member @16 gets vt[4] (shutdown) then vt[1] (delete) and is nulled; member @20 gets vt[1] (delete) and is nulled
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x2924 0x60 xy-decomp/armcc-matching/matched/DllBattle/0x2924.c
 */
typedef struct B { void (**vt)(struct B *); } B;
typedef struct O { void *pad[4]; B *a; B *b; } O;
void fn(O *o) {
    if (o->a) {
        o->a->vt[4](o->a);
        if (o->a) o->a->vt[1](o->a);
        o->a = 0;
    }
    if (o->b) {
        o->b->vt[1](o->b);
        o->b = 0;
    }
}
