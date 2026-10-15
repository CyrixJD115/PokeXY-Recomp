Module: DllSequence
Offset: 0x10a80
Size: 0x10
Match: 4/4
One-line: Chased-pointer "touch then always return true" accessor — loads this->inner (at +0xc), then inner->field (at +0xcc), discards the result, returns 1.
Note: This is the third flagged lead from the module's README entry (for 0x1135c) — but the real function starts at 0x10a80, not 0x10a88 (the earlier note quoted only the trailing stub words), and is 0x10 bytes not 0x8. New residue-class finding worth folding into docs/decomp/armcc-toolchain.md or a module note: armcc 5.04u5 at -O2 -Otime completely eliminates two chained, unused, non-volatile pointer loads (0/4 masked match, collapses to the bare stub) — marking the leaf field `volatile` is required and reproduces the target exactly (4/4). Worth checking other "untried lead" offsets flagged across other modules in case some of those are actually this same pattern (a leading pair of now-invisible dead loads) rather than genuine bare 2-word stubs.
