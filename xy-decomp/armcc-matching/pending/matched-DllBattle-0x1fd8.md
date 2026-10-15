Module: DllBattle
Offset: 0x1fd8
Size: 0xc
Match: 3/3
One-line: Field-getter-plus-offset -- loads pointer member at `this+2960` (0xb90), returns it +48.
Note: Fifth and last match of this run (hit the 5-function ceiling). The module's first substantial function (a dispatcher using many of the same +0xb.. fields seen in this stub cluster, `0x1fe4`-`0x2048`) was scanned but not attempted -- too large/stateful for the easy-win budget. DllBattle is a huge module (~0xcd1fc .text bytes, ~6461 functions per the heatmap) with its import-veneer table using an unusual "ldr pc,[pc,#-4]" absolute-literal idiom (895 entries, 0x180-0x1d8c) instead of the "b <target>" forwarder style seen in most other Dll modules -- worth noting for future agents mining this module. Untried easy leads nearby: twin `return 0` stub at 0x1fd0, 1-word tail-call forwarder at 0x1fcc (`b` into the veneer table), and the large VFP vector/matrix function at 0x1d8c-0x1f74 (rotation/quaternion-ish, not attempted).
