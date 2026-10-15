/*
 * DllBattle @ 0x1f8c, size 0x20 -- masked match 8/8.
 *
 * Zero-init helper: zeroes two leading u32 fields (offsets 0, 4) then four
 * trailing u16 fields (offsets 8, 10, 12, 14) of a 16-byte struct, returns
 * the (unchanged) pointer -- the common "constructor returns this" ARM C++
 * ABI shape, but written here as a free function since the body never
 * dereferences anything but the raw offsets.
 *
 * Non-obvious: statement order in the source controls armcc's emission
 * order here (no calls/branches to otherwise fix scheduling), and it is
 * NOT byte-offset order -- the retail binary emits the two u32 stores
 * first, THEN the four u16 stores, even though the u32s sit physically
 * after the u16 fields in memory in one naive reading. Declaring/assigning
 * the two raw int stores before the four short-field assignments in the C
 * source reproduced this exactly; the opposite order (shorts assigned
 * first) swaps the whole instruction sequence instead of just reordering
 * two groups.
 *
 * Found near DllBattle's large import-veneer table (895 entries,
 * 0x180-0x1d8c, using the module's "ldr pc, [pc, #-4]" absolute-literal
 * veneer idiom rather than the usual "b <target>" forwarder seen in most
 * other Dll modules) -- right after a big VFP vector-math function
 * (rotation/matrix build) ending at 0x1f74 plus its own literal pool
 * (0x1f78-0x1f8b).
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle \
 *     0x1f8c 0x20 xy-decomp/armcc-matching/matched/DllBattle/0x1f8c.c
 */

typedef struct {
    unsigned char pad[8];
    unsigned short a;
    unsigned short b;
    unsigned short c;
    unsigned short d;
} S;

void *fn(S *o) {
    *(int *)((char *)o + 0) = 0;
    *(int *)((char *)o + 4) = 0;
    o->a = 0;
    o->b = 0;
    o->c = 0;
    o->d = 0;
    return o;
}
