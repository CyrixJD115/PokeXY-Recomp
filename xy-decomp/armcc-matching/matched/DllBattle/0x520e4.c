/*
 * DllBattle @ 0x520e4, size 0x38 -- range-checked byte getter: returns o->arr[i*14] (u8 at +286, stride 14) when i < o->count (u8 at +338); otherwise calls assert stub fnA(0,0,0) and returns 0.
 * Masked match 14/14.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x520e4 0x38 xy-decomp/armcc-matching/matched/DllBattle/0x520e4.c
 */
extern void fnA(int a, int b, int c);
struct O { char pad[286]; unsigned char v[1]; };
struct Q { char pad[338]; unsigned char n; };
unsigned char fn(struct O *o, unsigned i) {
    if (i < ((struct Q*)o)->n)
        return *(unsigned char*)((char*)o + 286 + i*14);
    fnA(0,0,0);
    return 0;
}
