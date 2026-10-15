Module: DllCommWait
Offset: 0xcd8
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 0`.
Note: Eighth independent module mined. Same shape as DllConfig@0x3a74 / DllNumberInput@0x1ab0 / DllDialogCommon@0xd10 — part of the recurring trivial-stub cluster near a dialog-proc base class's vtable tail. Sits right after a destructor-stage pair (0xca8/0xc8c-style vptr-install helpers at 0xc74/0xca8, recognized on sight as the already-documented vptr-scheduling residue class and skipped) and right before a 5-case property get/set switch at 0xce0 (same jump-table shape as DllDialogCommon@0xdd8/DllNumberInput@0x1af8 — not attempted, same push{r4,lr}/nop-relaxation issue likely applies). Module also has three more lone `bx lr` single-word stubs at 0x2a0/0x2a4/0x2a8 (weak-evidence class, same caveat as prior modules) not yet claimed.
