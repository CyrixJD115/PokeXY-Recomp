Module: DllTestKawada
Offset: 0x180
Size: 0x24
Match: 9/9
One-line: init stub: volatile 64-bit global = -1, second 64-bit global = 0
Note: `volatile long long` is what makes armcc emit `strd` (plain long long -> stm). Identical bytes verified 9/9 in DllFieldDebug, DllTestIchiraku, DllTestNagihashi, DllTestSuzuki @0x180/0x24 (same source, not separately committed); DllTestKosaka 8/9 (strd r0,[r2,#8] -- A at +8 of a struct, one-word difference).
