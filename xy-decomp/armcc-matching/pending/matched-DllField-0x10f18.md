Module: DllField
Offset: 0x10f18
Size: 0xc
Match: 3/3
One-line: Byte-field setter (stores the *third* arg, r2 — second arg unused) into `this+141`, then unconditionally returns 1.
Note: Immediately follows the 0x10f10 stub in the same cluster; confirms the module has more than a one-arg-setter idiom — watch register roles (arg2 not arg1) on siblings.
