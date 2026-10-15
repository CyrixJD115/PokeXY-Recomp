Module: DllUSGymFighting
Offset: 0x11c0
Size: 0x1c
Match: 7/7
One-line: Struct zero-initializer — zeroes fields at +0/+8/+4 (that order) plus two byte flags at +12/+13.
Note: Same size gotcha seen elsewhere in this module (0x3f4): the function's
own trailing `bx lr` is part of its body, so real size is 0x1c, not the
naive 0x18 from the straight-line 6-instruction body alone.
