Module: DllCaptureRegister
Offset: 0x27b0
Size: 0x8
Match: 2/2
One-line: `xy_system::bg::System::SetIsCallDraw(bool)` byte-field setter (`strb r1,[r0,#217]; bx lr`), thirty-ninth module touched.
Note: Identical bytes to the already-matched DllTrainingAppManager@0xd68 / DllPssConfig@0x1c20 — same shared base/mixin-class method recurring across unrelated modules. Sits in a stub cluster at the module's import-veneer-table tail (0x2728-0x2844): two small dtor-shaped helpers at 0x2744/0x277c (vptr install + conditional free, skipped — not yet checked for the known-good/bad load-ordering residue class), then this setter, then two `return 1` stubs (0x27b8/0x27c0) and a long run of ~15 identical `return 0` stubs (0x27c8-0x2844) — none of those claimed yet, easy pickings for a future agent.
