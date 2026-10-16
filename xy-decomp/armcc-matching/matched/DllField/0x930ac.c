/*
 * DllField @ 0x930ac (size 0xd8) -- masked match, USING --mask-ranges 0x38:0x24 (9-word jump table).
 * Case-value-to-body mapping RECOVERED from the CRO internal relocations (tools/jumptable_relocs.py),
 * not proven by the compiler.
 * Script handler: fetches the mode byte block at mgr->+12->+28->+460, asks F(self,args[1],&out) for an output record, then writes out[0] from the mode (0/2/6 -> 0; 4/5/8 -> err + 1; 1/3 -> the mode itself; else err + mode) and copies the u32 at +4 and bytes +8/+9 into out[1..3]; returns 0.
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x930ac 0xd8 xy-decomp/armcc-matching/matched/DllField/0x930ac.c --mask-ranges 0x38:0x24
 */
extern char *GetMgr(void);
extern void F(int self, int idx, int **out);
extern void err(int, int, int);
int fn(int self, int *a)
{
    int *o;
    unsigned char *m = (unsigned char *)(*(char **)(*(char **)(GetMgr() + 12) + 28) + 460);
    F(self, a[1], &o);
    switch (m[0]) {
    case 4: case 5: case 8: err(0, 0, 0); o[0] = 1; break;
    case 0: case 2: case 6: o[0] = 0; break;
    case 1: case 3: o[0] = m[0]; break;
    default: err(0, 0, 0); o[0] = m[0]; break;
    }
    o[1] = *(int *)(m + 4);
    o[2] = m[8];
    o[3] = m[9];
    return 0;
}
