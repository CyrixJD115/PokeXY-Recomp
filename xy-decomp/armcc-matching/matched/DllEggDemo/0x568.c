// DllEggDemo@0x568, size 0x1c, masked match 7/7.
// Conditional accessor: a veneer call gates between a member load at
// offset 24 and a re-materialized constant 3. Same idiom already proven
// at DllLangSelect@0x5b0 (offset 20 / else 2), DllDendouDemo@0x798
// (offset 32 / else 2), and DllBoxSearch@0x5b8 (offset 32 / else 2) --
// matching pattern #6 from the toolchain's residue notes (the "obvious"
// boolean call result isn't reused; the false branch re-materializes a
// literal instead). This is the module's first real function, right
// after the import-veneer table (0x180-0x567).
//
// Verify:
//   XY_ROMFS=.Data/Source_Extracted/cro python3 \
//     xy-decomp/armcc-matching/tools/fingerprint_fn.py \
//     DllEggDemo 0x568 0x1c \
//     xy-decomp/armcc-matching/matched/DllEggDemo/0x568.c

typedef struct {
    unsigned char pad[24];
    int field;
} Obj;

extern int Check(Obj *p);

int GetOrDefault(Obj *p) {
    if (Check(p))
        return p->field;
    return 3;
}
