Module: DllFieldDemo
Offset: 0x1edc
Size: 0x8
Match: 2/2
One-line: `xy_system::bg::System::SetIsCallDraw(bool)` — trivial byte-field setter (`strb r1,[r0,#217]; bx lr`).
Note: Byte-identical to the already-matched instances of this shared base/mixin-class method in DllTrainingAppManager@0xd68, DllPssConfig@0x1c20, DllCaptureRegister@0x27b0 and DllColosseum@0x46fc — a 51st-area module confirming the same `+0xd9` field setter recurs verbatim. DllFieldDemo is a new (52nd) module for the match table. Sits right next to an untried `return 1` stub at 0x1ee4 and a small struct-zero init function at 0x1eec-0x1f0c, both plausible next easy wins for a future agent.
