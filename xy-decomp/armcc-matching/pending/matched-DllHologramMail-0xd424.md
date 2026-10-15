Module: DllHologramMail
Offset: 0xd424
Size: 0x24
Match: 9/9
One-line: Derefs `this+36` at +4 to get an owned object, calls an import veneer (0xd10) with the 3rd function argument and that object, stores literal 11 into `this+40`.
Note: Only the 3rd argument (r2) is used; the 2nd argument (r1) is dead/unused entirely. Has a byte-identical sibling at 0xd13c (same module, different masked bl target 0xd78) — see matched-DllHologramMail-0xd13c.md.
