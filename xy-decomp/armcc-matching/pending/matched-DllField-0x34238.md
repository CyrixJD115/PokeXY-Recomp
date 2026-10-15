Module: DllField
Offset: 0x34238
Size: 0x10
Match: 4/4
One-line: Two fixed no-arg calls, void return -- FunA(); FunB(); (second call tail-called via b).
Note: Real size is 0x10 (4 words: push {r4,lr}; bl; pop {r4,lr}; b), not the hinted 0x34 -- the hint span overran into the next function's body (field setters at 0x3424c onward). The push/pop of r4 is AAPCS stack-alignment padding, not a real save.
