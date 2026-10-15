Module: DllSalon
Offset: 0x380
Size: 0x20
Match: 8/8
One-line: boolean predicate — returns 1 if a private helper at 0xaa0 (called on this->f48 + 4) yields 0, else 0
Note: confirms the same +0x48-then-+4 accessor pattern as the rest of this module's early functions; the 0xaa0 callee is an in-module function, masked by the verifier
