Module: DllTestPGL
Offset: 0x180
Size: 0x8
Match: 2/2
One-line: xy_system::bg::System::SetIsCallDraw(bool) byte-field setter; stores r1 at this+0xd9. Confirmed via module's named export _ZN9xy_system2bg6System13SetIsCallDrawEb.
Note: Same recurring shared-base setter pattern seen in DllTrainingAppManager@0xd68, DllPssConfig@0x1c20, DllCaptureRegister@0x27b0, DllColosseum@0x46fc, DllFieldDemo@0x1edc, DllIntro@0x9eb0, DllGts@0x1b7e4. DllTestPGL's entire .text is only 0x180 bytes; the rest is an nnroControlObject_ (same jump-table residue class as DllDialogCommon@0xdd8).