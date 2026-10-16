// DllDendouPC@0x650, size 0x1c, masked match 7/7.
// Conditional accessor: a veneer call gates between a member load at
// offset 36 and a re-materialized constant 2. Same idiom already proven at
// DllLangSelect@0x5b0 (member offset 20), DllDendouDemo@0x798 (offset 32),
// and DllBoxSearch@0x5b8 (offset 32) -- matching pattern #6 from the
// toolchain's residue notes (the "obvious" boolean result isn't reused;
// the false branch re-materializes a literal instead).
// This is the module's first real function, right after the import-veneer
// table (0x180-0x61f) and a run of trivial return-1/bare-bx-lr stubs
// (0x620-0x64f).
// Style: natural
//
// Verify:
//   XY_ROMFS=.Data/Raw_Extracted/romfs python3 \
//     xy-decomp/armcc-matching/tools/fingerprint_fn.py \
//     DllDendouPC 0x650 0x1c \
//     xy-decomp/armcc-matching/matched/DllDendouPC/0x650.c

typedef struct {
    unsigned char pad[36];
    int field;
} Obj;

extern int Check(Obj *p);

int GetOrDefault(Obj *p) {
    if (Check(p))
        return p->field;
    return 2;
}
