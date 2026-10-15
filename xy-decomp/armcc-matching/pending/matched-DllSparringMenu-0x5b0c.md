Module: DllSparringMenu
Offset: 0x5b0c
Size: 0x8
Match: 2/2
One-line: xy_system::bg::System::SetIsCallDraw(bool) — byte-field setter, void return.
Note: Confirmed by the module's own named export (`_ZN9xy_system2bg6System13SetIsCallDrawEb`,
symbols.tsv load_addr 0x105b0c). Identical bytes/shape to the same shared setter already
matched in DllTrainingAppManager@0xd68, DllPssConfig@0x1c20, DllCaptureRegister@0x27b0,
DllColosseum@0x46fc, DllFieldDemo@0x1edc, DllIntro@0x9eb0, DllGts@0x1b7e4 — an eighty-fifth
module confirming this shared base/mixin-class method recurs verbatim.
