/*
 * DllBattle @ 0x60e00 (size 0x74) -- masked match 29/29, USING --mask-ranges 0x14:0x18 (jump table).
 * Case-value-to-body mapping: the mask alone cannot prove it, but it was RECOVERED from the CRO's
 * internal relocations (tools/jumptable_relocs.py DllBattle 0x60e14 0x60e2c): slot 0 -> default/err block,
 * slots 1,3,4,5 -> "r=1" body, slot 2 -> "pack" body (this source reproduces that: cases 1,3,4,5 share a
 * body; selector 0 and >=6 hit the error stub). Relocation-derived, not compiler-proven.
 * What it does: maps a small selector to a packed code: 1 for selectors 1/3/4/5; for 2:
 * 2 | (((calc(g_inst.f54, 3) + 2) << 3) & 0x1f8) (calc = import call, g_inst = a relocated global
 * object read at +0x54); selector 0 / out of range call the game's assert/error stub err(0,0,0), return 0.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBattle 0x60e00 0x74 xy-decomp/armcc-matching/matched/DllBattle/0x60e00.c --mask-ranges 0x14:0x18
 */
typedef struct S { char pad[0x54]; int f; } S;
extern S g_inst;
extern int calc(int, int);
extern void err(int, int, int);
int fn(int k)
{
    int r = 0;
    switch (k) {
    case 1: case 3: case 4: case 5: r = 1; break;
    case 2: r = ((calc(g_inst.f, 3) + 2) << 3 & 0x1f8) | 2; break;
    default: err(0, 0, 0); break;
    }
    return r;
}
