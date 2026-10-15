Module: DllStaffRoll
Offset: 0x2cb0
Size: 0x8
Match: 2/2
One-line: `xy_system::bg::System::SetIsCallDraw(bool)` — byte-field setter at +0xd9.
Note: Same shared base/mixin-class method already matched in DllTrainingAppManager@0xd68/DllPssConfig@0x1c20/DllCaptureRegister@0x27b0/DllColosseum@0x46fc/DllFieldDemo@0x1edc/DllIntro@0x9eb0 — adds DllStaffRoll to that recurring cross-module method's list. An untried `return 0` stub sits right next door at 0x2cb8.

This was my 5th match this run (budget reached). This module (DllStaffRoll) still has plenty unmined: a double-vtable-install dtor (base-dtor-call + member-free + second vtable install) around 0x28c8/0x289c-0x28bc not attempted (more complex than the simple single-vtable idiom — two `bl` calls to local destructors before the member-free), the 5-case jump-table dispatcher at 0x5d0 (known-hard residue class, skip), and a large constructor-ish region further in. Good next candidates for a sibling agent: 0x2cb8 (`return 0` stub, trivial), and the 0x28c8/0x289c pair if someone wants to try the double-vtable-install dtor shape.
