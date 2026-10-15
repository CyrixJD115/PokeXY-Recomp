Module: DllHologramMail
Offset: 0xd13c
Size: 0x24
Match: 9/9
One-line: Byte-identical shape to 0xd424 in the same module (masked bl target resolves to 0xd78 instead of 0xd10) — reused the 0xd424 candidate unmodified (just the import's name changed for readability).
Note: Confirms the "deref this+36 at +4, call import with 3rd arg + object, store 11 into this+40" idiom recurs more than once per module.
