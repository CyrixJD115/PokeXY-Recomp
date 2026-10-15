/* DllTestPGL@0x180 - 2/2 match
 * xy_system::bg::System::SetIsCallDraw(bool) - byte-field setter.
 * Stores r1 as a byte at this+0xd9 (217), void return.
 * Same byte-pattern recurring in: DllTrainingAppManager@0xd68,
 * DllPssConfig@0x1c20, DllCaptureRegister@0x27b0, DllColosseum@0x46fc,
 * DllFieldDemo@0x1edc, DllIntro@0x9eb0, DllGts@0x1b7e4. Module's named
 * export table confirms: _ZN9xy_system2bg6System13SetIsCallDrawEb.
 * Verify: python3 tools/fingerprint_fn.py DllTestPGL 0x180 0x8 matched/DllTestPGL/0x180.c
 */
void SetIsCallDraw(unsigned char *this, int v) {
    *(unsigned char *)(this + 0xd9) = (unsigned char)v;
}