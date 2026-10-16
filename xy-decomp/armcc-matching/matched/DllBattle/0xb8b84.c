/*
 * DllBattle @ 0xb8b84, size 0x50 -- battle event trigger: when query(3)==c and (u8)query(4)!=31 and check(c), posts helper(66,1)
 * Style: trick(static-helper) -- one-use static helper taking (c, v) only so the u8 query lands in r1 (and r1,r0,#255) as retail; inline `v != 31 && f4c5a8(c)` form puts it in r0 (audit 2026-10-16: tested, mismatches)
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0xb8b84 0x50 xy-decomp/armcc-matching/matched/DllBattle/0xb8b84.c
 */
extern int f6b91c(int);
extern int f4c5a8(int);
extern void f658d4(int, int);
static int chk(int c, int v) { return v != 31 && f4c5a8(c); }
void fn(int a, int b, int c) {
    if (f6b91c(3) == c && chk(c, (unsigned char)f6b91c(4))) f658d4(66, 1);
}
