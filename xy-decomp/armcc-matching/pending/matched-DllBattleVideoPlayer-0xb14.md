Module: DllBattleVideoPlayer
Offset: 0xb14
Size: 0x4
Match: 1/1
One-line: Prologue-less single-instruction tail-call forwarder (`b <import veneer>`), same thin-forwarder idiom as DllBattleVideoSave@0x454.
Note: 46th module touched. Sits in a run of 8 near-identical one-word leaf
functions (0xb14-0xb33: alternating `b 0x188` forwarders and bare `bx lr`
no-op stubs) right after a veneer-calling wrapper ending at 0xb0c (own
literal pool at 0xb10) and before the next full-prologue function at
0xb34. The other 7 stubs in this run (0xb18, 0xb1c, 0xb20, 0xb24, 0xb28,
0xb2c, 0xb30) are untried and would be easy next picks for another agent.
Module's import-veneer table runs 0x180-0xac7 (full `.text` segment is
file offset 0x180, size 0xea00 per `xy cro info`); first real function
after the veneer table (0xac8) is a dispatcher, then another at 0xaf0,
then this stub cluster.
