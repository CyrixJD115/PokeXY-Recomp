Module: DllKawaigari
Offset: 0x3fc6c
Size: 0x30
Match: 12/12
One-line: No-arg helper — if a nested global pointer chain (g_outer.mid->inner) is non-null, sets its +0x24 int field to 1, then unconditionally sets a second global byte flag to 1; returns 1.
Note: A new (sixty-third) module. Prologue-less leaf with no incoming-argument use at all — genuinely zero-argument, not a stub (both r0 and r1 are set from literals/immediates inside the body, never read from the caller). Quoted size (0x30) includes the function's own 2-word literal pool (both reloc placeholders, 0 pre-link), same convention as the dtor-idiom matches elsewhere. Matched on the first attempt; no residue-class issues hit. The module's `.text` is large (0x43bbc bytes) with an unusually long import-veneer table (ends ~0xd84); this candidate sits right after the initial stub cluster (0xd88-0xdd8-ish thin forwarders) — plenty of unmined real functions remain in this module for a future agent.
