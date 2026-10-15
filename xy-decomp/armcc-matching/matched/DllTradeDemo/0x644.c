// DllTradeDemo@0x644, size 0x24, masked match 9/9.
// Prologue-less leaf predicate: double-indirects through a pointer field
// at +24 to a Leaf* that itself carries a byte flag at +56; when that
// flag is 1, conditionally stores 5 into a field at +16 and returns 2,
// else returns 4 (no store). Immediately follows DllTradeDemo@0x550's
// literal pool (one word at 0x640) and precedes a push{r4,lr}-framed
// conditional-free helper at 0x668.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
//     DllTradeDemo 0x644 0x24 \
//     xy-decomp/armcc-matching/matched/DllTradeDemo/0x644.c

typedef struct {
    unsigned char pad[56];
    unsigned char flag;
} Leaf;

typedef struct {
    unsigned char pad0[16];
    int state;
    unsigned char pad1[4];
    Leaf **leafptr;
} This;

int Check(This *t) {
    Leaf *l = *t->leafptr;
    if (l->flag == 1) {
        t->state = 5;
        return 2;
    }
    return 4;
}
