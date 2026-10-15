Module: DllDendouDemo
Offset: 0x798
Size: 0x1c
Match: 7/7
One-line: Calls an import veneer with `this` unchanged; returns the field at +32 if the call's result is nonzero, else the literal 2.
Note: Forty-third module mined. First real function right after the import-veneer table (0x180-0x797). Matched on the first attempt — pattern #3 from the executor skill ("simple field-copy-and-call helpers"). The module's second function (0x7b4, a larger multi-case dispatcher with VFP float math) was not attempted.
