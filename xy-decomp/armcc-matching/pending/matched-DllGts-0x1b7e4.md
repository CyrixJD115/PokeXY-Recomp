Module: DllGts
Offset: 0x1b7e4
Size: 0x8
Match: 2/2
One-line: `xy_system::bg::System::SetIsCallDraw(bool)` (named export, demangled from `_ZN9xy_system2bg6System13SetIsCallDrawEb`) — a trivial boolean-field setter, `strb r1,[r0,#217]; bx lr`.
Note: First matched function in DllGts (previously unmined module — only DllDialogCommon had matches). Found via the module's own named exports (`xy cro extract` → `symbols.tsv`), not blind disassembly scanning — the export was already a demangled C++ method name, which made the member offset (0xd9) obvious from the single `strb` immediate. Modeled as `unsigned char pad[217]` followed by the real `bool` member to hit the exact byte offset; compiled as `.cpp` (armcc, `--cpp` auto-added by the verifier for that extension).
