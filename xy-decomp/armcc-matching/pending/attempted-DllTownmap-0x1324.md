Module: DllTownmap
Offset: 0x1324 (size 0x38)
What: bounds-checked accessor: if (this->[4]->[0] <= idx) bl fail(0,0,0); return this->[0xc] + idx*8.
Tried: struct-field C, hoisted count load, if/else forms, pointer-arith forms. All 9/14: instruction stream identical except register allocation -- retail keeps this in r4 and idx in r5, armcc gives this in r5, idx in r4. Unresolved.
