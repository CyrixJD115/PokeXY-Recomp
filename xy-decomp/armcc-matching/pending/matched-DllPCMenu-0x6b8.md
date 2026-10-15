Module: DllPCMenu
Offset: 0x6b8
Size: 0x1c
Match: 7/7
One-line: Conditional accessor — calls an import veneer (0x1b8) with `this` unchanged; returns member field at +32 if the result is nonzero, else the literal 2.
Note: A new (sixty-third+) module for this family of specimen. Same idiom already proven in DllLangSelect@0x5b0 / DllDendouDemo@0x798 / DllBoxSearch@0x5b8 / DllDendouPC@0x650 / DllEggDemo@0x568 (veneer-gated member-load vs re-materialized constant). Sits right after the module's first real function (0x550-0x6b0, an unattempted larger dispatcher/state-machine function) and that function's own one-word literal pool at 0x6b4. Matched on the first attempt.
