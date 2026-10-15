Module: DllTradeDemo
Offset: 0x550
Size: 0x1c
Match: 7/7
One-line: Conditional accessor (veneer call gates member load at +20 vs re-materialized constant 3) — same idiom as DllLangSelect@0x5b0/DllDendouDemo@0x798/DllBoxSearch@0x5b8/DllDendouPC@0x650/DllEggDemo@0x568/DllPCMenu@0x6b8.
Note: Module's first real function, right after the import-veneer table (0x180-0x54f). This is the seventy-first-ish independent module for this idiom (opening a new module in the matched/ list). Gotcha for the next agent on this module: `xy cro info`'s reported segment offsets (e.g. "seg 0: .text @0x180") are already absolute file offsets into the .cro — do NOT add the CRO0 header's 0x80 on top when extracting/disassembling by hand (fingerprint_fn.py's own `offset` arg is likewise a raw absolute file offset).
