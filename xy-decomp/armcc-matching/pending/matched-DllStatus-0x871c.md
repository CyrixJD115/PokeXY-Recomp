Module: DllStatus
Offset: 0x871c
Size: 0x4
Match: 1/1
One-line: Thin tail-call forwarder -- entire body is a single `b <import veneer>` instruction.
Note: Same idiom as this module's own 0x6f6c match. Sits right after a
destructor-shaped function ending at 0x8710 (literal pool through
0x871b), and right before a bare `bx lr` single-word stub (0x8720,
unclaimed).
