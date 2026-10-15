Module: DllUSPassage
Offset: 0x1610
Size: 0x4
Match: 1/1
One-line: Thin tail-call forwarder — entire body is a single `b <import veneer>` instruction.
Note: Same idiom as this module's own 0x1078. This module has four near-
identical copies of this forwarder (0x1078, 0x1610, 0x1c80, 0x20c4) — the
latter two are untried, unclaimed easy follow-ups for the next agent.
