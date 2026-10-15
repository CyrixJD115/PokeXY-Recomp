Module: DllField
Offset: 0x5445c
Size: 0x38
Match: 14/14
One-line: Call+return helper -- calls two import veneers with (this, byte@10, 0, 2, 8) then (field0, 33), returns 1.
Note: find_candidates.py's reported size (0x40) included two trailing one-word stubs after this function's real end (0x54490); true size is 0x38. Argument evaluation matched armcc's right-to-left convention on the first try.
