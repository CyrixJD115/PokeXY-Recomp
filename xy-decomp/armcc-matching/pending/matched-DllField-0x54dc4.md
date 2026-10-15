Module: DllField
Offset: 0x54dc4
Size: 0x18
Match: 6/6
One-line: Call+return helper -- calls import veneer with (field0, 16), returns hardcoded 3.
Note: Found by scanning right after the 0x54d84/0x5445c pair. Easy follow-up right next door: 0x54de4 (same shape, veneer called with (field0, 0, 8), also returns 3) -- not attempted, left for a future agent.
