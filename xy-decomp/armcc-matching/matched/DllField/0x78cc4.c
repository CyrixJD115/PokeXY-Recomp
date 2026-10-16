/* DllField function @ CRO file offset 0x78cc4, size 0x5c -- MATCHED (masked, 23/23).
 * find-by-key: scans 33 slots (stride 0x90) for a live entry (+20 or +28 flag set) whose key (+0) equals the argument; returns the address of that slot's entry pointer, or null
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x78cc4 0x5c xy-decomp/armcc-matching/matched/DllField/0x78cc4.c
 */
typedef struct E { int key; char pad[16]; unsigned char f20; char pad2[7]; unsigned char f28; } E;
typedef struct Slot { char pad[64]; E *e; char pad2[76]; } Slot;
E **fn(Slot *s, int key)
{
    unsigned i;
    for (i = 0; i < 33; i++) {
        E *e = s[i].e;
        if (e && (e->f20 || e->f28) && e->key == key)
            return &s[i].e;
    }
    return 0;
}
