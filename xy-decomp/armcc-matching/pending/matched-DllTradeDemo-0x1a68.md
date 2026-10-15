Module: DllTradeDemo
Offset: 0x1a68
Size: 0x8
Match: 2/2
One-line: xy_system::bg::System::SetIsCallDraw(bool) byte-field setter (strb r1,[r0,#217]; bx lr) -- identical bytes to DllTrainingAppManager@0xd68/DllPssConfig@0x1c20/DllCaptureRegister@0x27b0/DllColosseum@0x46fc/DllFieldDemo@0x1edc/DllIntro@0x9eb0.
Note: Right next to a trivial return-1 stub (0x1a70, also matched this run) and right before a 64-bit-globals-init leaf (0x1a78) that matches the known dead-end residue class already documented in attempted.md (DllFieldDebug@0x180: value-computed-first/strd-pair ordering an isolated compile can't reproduce) -- recognized on sight and skipped, not re-attempted.
