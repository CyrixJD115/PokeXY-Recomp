/*
 * DllField @ 0x957fc (size 0x84) -- masked match, USING --mask-ranges 0x20:0x24 (9-word jump table).
 * Case-value-to-body mapping RECOVERED from the CRO internal relocations (tools/jumptable_relocs.py),
 * not proven by the compiler.
 * Maps the mode byte at mgr->+12->+28->+460 to a result: 1->1, 3->2, 4->3, 5->4, 8->err(0,0,0) then 1, anything else -> 0.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x957fc 0x84 xy-decomp/armcc-matching/matched/DllField/0x957fc.c --mask-ranges 0x20:0x24
 */
typedef struct C { char pad[460]; unsigned char m; } C;
typedef struct B { char pad[28]; C *c; } B;
typedef struct A { char pad[12]; B *b; } A;
extern A *inst(void);
extern void err(int, int, int);
int fn(void)
{
    switch (inst()->b->c->m) {
    case 1: return 1;
    case 3: return 2;
    case 4: return 3;
    case 5: return 4;
    case 8: err(0, 0, 0); return 1;
    default: return 0;
    }
}
