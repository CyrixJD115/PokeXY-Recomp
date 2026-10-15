Module: DllUSGymIce
Offset: 0x1974
Size: 0x2c
Match: 11/11
One-line: globals init (-1 to a volatile 64-bit global via strd, 0 to a second via stm)
Note: first global must be volatile long long (gives strd); Kosaka uses volatile long long g_a[2]; g_a[1] (strd to [r2,#8]). Pool words are relocations (zero in both).
