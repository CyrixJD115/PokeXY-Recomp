Module: DllStartMenu
Offset: 0x1b7c
Size: 0x4
Match: 1/1
One-line: Trivial no-op void stub (single `bx lr`) — weak evidence, see note below.
Note: one of a run of five identical 1-word `bx lr` stubs at 0x1b7c-0x1b8c (likely default virtual no-op overrides); only 0x1b7c claimed here, siblings 0x1b80/0x1b84/0x1b88/0x1b8c are free easy follow-ups for a future agent.
