// DllTradeDemo@0x550, size 0x1c, masked match 7/7.
// Conditional accessor: a veneer call gates between a member load at
// offset 20 and a re-materialized constant 3. Same idiom already proven
// at DllLangSelect@0x5b0 (offset 20 / else 2), DllDendouDemo@0x798
// (offset 32 / else 2), DllBoxSearch@0x5b8 (offset 32 / else 2),
// DllDendouPC@0x650 (offset 36 / else 2), DllEggDemo@0x568 (offset 24 /
// else 3), DllPCMenu@0x6b8 -- matching pattern #6 from the toolchain's
// residue notes (the "obvious" boolean call result isn't reused; the
// false branch re-materializes a literal instead). Module's first real
// function, right after the import-veneer table (0x180-0x54f).
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
//     DllTradeDemo 0x550 0x1c \
//     xy-decomp/armcc-matching/matched/DllTradeDemo/0x550.c

typedef struct {
    unsigned char pad[20];
    int field;
} Obj;

extern int Check(Obj *p);

int GetOrDefault(Obj *p) {
    if (Check(p))
        return p->field;
    return 3;
}
