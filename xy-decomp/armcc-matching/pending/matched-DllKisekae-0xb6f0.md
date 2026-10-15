Module: DllKisekae
Offset: 0xb6f0
Size: 0xc
Match: 3/3
One-line: Byte-field setter: stores literal 1 at this+0x40a (1034), void return.
Note: Source-phrasing gotcha: passing the literal via a local `int v = 1;` keeps the redundant `mov r1, #1` instruction — armcc 5.04u5 at -O2 -Otime otherwise fuses the immediate into the strb encoding (2 words), missing the retail's explicit-materialization pattern.