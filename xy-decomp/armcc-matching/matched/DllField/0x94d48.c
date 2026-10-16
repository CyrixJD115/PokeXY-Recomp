/*
 * DllField @ 0x94d48 (size 0x6c) -- masked match 27/27, USING --mask-ranges 0x20:0x24 (9-word jump table).
 * Case-value-to-body mapping RECOVERED from the CRO internal relocations (tools/jumptable_relocs.py
 * DllField 0x94d68 0x94d8c), not proven by the compiler: v=0,2,6 -> return 0; v=8 -> return 1;
 * v=1,3 -> straight to the shared "return 2"; v=4,5,7 and v>=9 -> err(0,0,0) then return 2.
 * What it does: reads a mode byte at inst()->+12->+28->+460 and classifies it into a 0/1/2 result
 * (inst = import accessor at 0x16c0, err = the game's assert/error stub).
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x94d48 0x6c xy-decomp/armcc-matching/matched/DllField/0x94d48.c --mask-ranges 0x20:0x24
 */
typedef struct C { char pad[460]; unsigned char m; } C;
typedef struct B { char pad[28]; C *c; } B;
typedef struct A { char pad[12]; B *b; } A;
extern A *inst(void);
extern void err(int, int, int);
int fn(void)
{
    switch (inst()->b->c->m) {
    case 8: return 1;
    case 0: case 2: case 6: return 0;
    case 1: case 3: break;
    default: err(0, 0, 0); break;
    }
    return 2;
}
