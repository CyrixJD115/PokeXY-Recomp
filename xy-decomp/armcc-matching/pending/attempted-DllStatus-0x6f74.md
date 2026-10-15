DllStatus 0x6f74 (size 0x1c): copies 32 bytes src+0 -> dst+32 via vldmia/vstmia s0-s7, then 8 bytes src+32 -> dst+64 via ldr [36], ldr [32], str, str.
Tried: struct assignment of u32[8] (ldm/stmia with push), float[8] struct assign (same), loop, unrolled float assigns (vldr/vstr), V4 structs. None produce vldmia. Unsolved.
