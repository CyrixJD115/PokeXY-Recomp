Module: DllField
Offset: 0x54d84
Size: 0x38
Match: 14/14
One-line: Call+return helper, sibling of 0x5445c -- calls (field0, 196) then (this, byte@10, 1, 1, 12), returns 1.
Note: Same find_candidates.py size-rounding note as 0x5445c (reported 0x40 includes two trailing stub words; real size 0x38). Residue worth recording: the two identical-value arguments (r2=r3=1) only matched when written as the bare literal `1, 1` in the call -- a named local variable holding 1 used twice compiled with r2/r3's materialize-vs-copy roles swapped (armcc then puts the fresh materialize in r2 and copies to r3, while retail materializes r3 first and copies to r2).
