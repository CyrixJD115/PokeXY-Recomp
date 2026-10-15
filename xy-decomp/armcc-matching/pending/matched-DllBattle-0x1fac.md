Module: DllBattle
Offset: 0x1fac
Size: 0x10
Match: 4/4
One-line: Boolean predicate `return x == 0;` on the 2nd argument, ignoring `this`.
Note: Immediately follows the 0x1f8c zero-init match in the same module. No scheduling trap here -- straightforward cmp/moveq/movne compile.
