Module: DllTrialHouseResult
Offset: 0x2648
Size: 0x38
Match: 14/14
One-line: destructor -- vptr install, frees member +0x14 via masked veneer call if non-null, zeroes it, tail-calls another veneer with `this`.
Note: to get the exact instruction order (literal load, member load, store, THEN compare -- the "member-load after vtable-pool-load" good shape) the vtable literal and member both need separate named locals pulled up front (`void *vt = VT; void *m = a1->member;`) before use; plain sequential assignment+if statements make armcc reuse one register for both (store-then-load dependency), serializing them in the wrong order and capping the match at 11/14. Worth folding into the README's destructor-pattern note.
